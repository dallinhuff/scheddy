use crate::postgres::Postgres;
use application::offering::ports::repository::{Error, OfferingRepository};
use domain::offering::{Offering, OfferingId, Rental, Tour, TourRental};
use domain::vendor::VendorId;
use serde::Deserialize;
use sqlx::{PgPool, query};

impl OfferingRepository for Postgres {
    async fn get_offering_by_id(&self, id: OfferingId) -> Result<Option<Offering>, Error> {
        if let Some(tour) = get_tour_by_id(&self.pool, id).await? {
            Ok(Some(Offering::Tour(tour)))
        } else if let Some(rental) = get_rental_by_id(&self.pool, id).await? {
            Ok(Some(Offering::Rental(rental)))
        } else {
            Ok(None)
        }
    }

    async fn get_offerings_by_vendor(&self, vendor_id: VendorId) -> Result<Vec<Offering>, Error> {
        let tours = self
            .get_tours_by_vendor(vendor_id)
            .await?
            .into_iter()
            .map(Offering::Tour);
        let rentals = self
            .get_rentals_by_vendor(vendor_id)
            .await?
            .into_iter()
            .map(Offering::Rental);

        Ok(tours.chain(rentals).collect())
    }

    async fn get_tours_by_vendor(&self, vendor_id: VendorId) -> Result<Vec<Tour>, Error> {
        query!(
            r#"WITH rentals_by_tour AS (
                  SELECT tour_id,
                         json_agg(json_build_object('rental_id', rental_id, 'title', title)) AS rentals
                  FROM tour_rental
                       JOIN rental USING (rental_id)
                       JOIN offering ON rental_id = offering_id
                  GROUP BY tour_id
               )
               SELECT tour_id,
                      title,
                      coalesce(rentals, '[]'::json) as "rentals!"
               FROM tour
                    JOIN offering on tour_id = offering_id
                    LEFT JOIN rentals_by_tour USING (tour_id)
               WHERE vendor_id = $1"#,
            vendor_id.inner()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?
        .into_iter()
        .map(|t| {
            Ok(Tour {
                id: OfferingId(t.tour_id),
                vendor: vendor_id,
                title: t.title,
                rentals: Vec::deserialize(t.rentals).unwrap(),
            })
        })
        .collect()
    }

    async fn get_rentals_by_vendor(&self, vendor_id: VendorId) -> Result<Vec<Rental>, Error> {
        let rentals = query!(
            r#"SELECT rental_id, title
               FROM rental JOIN offering ON rental_id = offering_id
               WHERE vendor_id = $1"#,
            vendor_id.inner()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?
        .into_iter()
        .map(|r| Rental {
            id: OfferingId(r.rental_id),
            vendor: vendor_id,
            title: r.title,
        })
        .collect();

        Ok(rentals)
    }

    async fn save(&self, offering: Offering) -> Result<Offering, Error> {
        let saved_offering = match &offering {
            Offering::Tour(tour) => Offering::Tour(save_tour(&self.pool, tour).await?),
            Offering::Rental(rental) => Offering::Rental(save_rental(&self.pool, rental).await?),
        };

        Ok(saved_offering)
    }

    async fn delete(&self, id: OfferingId) -> Result<(), Error> {
        query!(
            "DELETE FROM offering WHERE offering_id = $1 RETURNING offering_id",
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(())
    }
}

async fn get_tour_by_id(pool: &PgPool, id: OfferingId) -> Result<Option<Tour>, Error> {
    let tour = query!(
        r#"SELECT tour_id, vendor_id, title
           FROM tour JOIN offering ON tour_id = offering_id
           WHERE tour_id = $1"#,
        id.0
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    let Some(tour) = tour else { return Ok(None) };
    let rentals = get_tour_rentals(pool, id).await?;

    Ok(Some(Tour {
        id: OfferingId(tour.tour_id),
        title: tour.title,
        vendor: tour.vendor_id.into(),
        rentals,
    }))
}

async fn get_tour_rentals(pool: &PgPool, tour_id: OfferingId) -> Result<Vec<TourRental>, Error> {
    query!(
        r#"SELECT rental_id, title
           FROM tour_rental
             JOIN rental USING (rental_id)
             JOIN offering ON rental_id = offering_id
           WHERE tour_id = $1"#,
        tour_id.0
    )
    .fetch_all(pool)
    .await
    .map_err(|e| Error::Unknown(e.into()))?
    .into_iter()
    .map(|r| {
        Ok(TourRental {
            id: OfferingId(r.rental_id),
            title: r.title,
        })
    })
    .collect()
}

async fn get_rental_by_id(pool: &PgPool, rental_id: OfferingId) -> Result<Option<Rental>, Error> {
    query!(
        r#"SELECT vendor_id, title
           FROM rental JOIN offering ON rental_id = offering_id
           WHERE rental_id = $1"#,
        rental_id.0
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| Error::Unknown(e.into()))?
    .map(|r| {
        Ok(Rental {
            id: rental_id,
            vendor: r.vendor_id.into(),
            title: r.title,
        })
    })
    .transpose()
}

async fn save_tour(pool: &PgPool, tour: &Tour) -> Result<Tour, Error> {
    let mut txn = pool.begin().await.map_err(|e| Error::Unknown(e.into()))?;

    query!(
        r#"INSERT INTO offering (offering_id, vendor_id, title)
           VALUES ($1, $2, $3)
           ON CONFLICT (offering_id)
              DO UPDATE SET (vendor_id, title) = ($2, $3)
           RETURNING offering_id"#,
        tour.id.0,
        tour.vendor.inner(),
        tour.title
    )
    .fetch_one(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    query!(
        r#"INSERT INTO tour (tour_id) VALUES ($1) ON CONFLICT (tour_id) DO NOTHING"#,
        tour.id.0
    )
    .execute(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    query!("DELETE FROM tour_rental WHERE tour_id = $1", tour.id.0)
        .execute(&mut *txn)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

    query!(
        "INSERT INTO tour_rental (tour_id, rental_id) SELECT * FROM UNNEST($1::uuid[], $2::uuid[])",
        &tour.rentals.iter().map(|_| tour.id.0).collect::<Vec<_>>(),
        &tour.rentals.iter().map(|r| r.id.0).collect::<Vec<_>>(),
    )
    .execute(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    txn.commit().await.map_err(|e| Error::Unknown(e.into()))?;

    Ok(tour.clone())
}

async fn save_rental(pool: &PgPool, rental: &Rental) -> Result<Rental, Error> {
    let mut txn = pool.begin().await.map_err(|e| Error::Unknown(e.into()))?;

    let res = query!(
        r#"INSERT INTO offering (offering_id, vendor_id, title)
           VALUES ($1, $2, $3)
           ON CONFLICT (offering_id)
              DO UPDATE SET (vendor_id, title) = ($2, $3)
           RETURNING offering_id AS rental_id, vendor_id, title"#,
        rental.id.0,
        rental.vendor.inner(),
        rental.title
    )
    .fetch_one(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    query!(
        "INSERT INTO rental (rental_id) VALUES ($1) ON CONFLICT (rental_id) DO NOTHING",
        rental.id.0
    )
    .execute(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    txn.commit().await.map_err(|e| Error::Unknown(e.into()))?;

    Ok(Rental {
        id: OfferingId(res.rental_id),
        vendor: res.vendor_id.into(),
        title: res.title,
    })
}
