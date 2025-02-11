use crate::postgres::Postgres;
use application::offering::ports::repository::{Error, OfferingRepository};
use domain::offering::{Offering, OfferingId, Rental, Tour, TourRental, TourStyle};
use domain::vendor::VendorId;
use serde::Deserialize;
use sqlx::{query, PgPool};

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
            "
            WITH rentals_by_tour AS
            (
                SELECT tour_id AS id,
                       json_agg(json_build_object('id', rental_id, 'name', name)) AS rentals
                FROM tour_rental
                     JOIN rental ON rental_id = rental.id
                     JOIN offering ON rental_id = offering.id
                     GROUP BY tour_id
            )
            SELECT id, name, style, coalesce(rentals, '[]'::json) as rentals
            FROM tour
                JOIN offering USING (id)
                LEFT JOIN rentals_by_tour USING (id)
            WHERE vendor_id = $1
            ",
            vendor_id.inner(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?
        .into_iter()
        .map(|t| {
            Ok(Tour {
                id: OfferingId(t.id),
                vendor: vendor_id,
                name: t.name,
                style: TourStyle::Guided,
                rentals: Vec::deserialize(t.rentals.unwrap())
                    .map_err(|e| Error::Unknown(e.into()))?,
            })
        })
        .collect()
    }

    async fn get_rentals_by_vendor(&self, vendor_id: VendorId) -> Result<Vec<Rental>, Error> {
        let rentals = query!(
            "SELECT id, name FROM offering JOIN rental USING (id) WHERE vendor_id = $1",
            vendor_id.inner()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?
        .into_iter()
        .map(|r| Rental {
            id: OfferingId(r.id),
            vendor: vendor_id,
            name: r.name,
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
        query!("DELETE FROM offering WHERE id = $1 RETURNING id", id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Unknown(e.into()))
            .map(|_| ())
    }
}

async fn get_tour_by_id(pool: &PgPool, id: OfferingId) -> Result<Option<Tour>, Error> {
    let tour = query!(
        "SELECT id, vendor_id, name, style FROM offering JOIN tour USING (id) WHERE id = $1",
        id.0
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    let Some(tour) = tour else { return Ok(None) };
    let rentals = get_tour_rentals(pool, id).await?;

    Ok(Some(Tour {
        id: OfferingId(tour.id),
        name: tour.name,
        vendor: tour.vendor_id.into(),
        style: match tour.style {
            n if n == TourStyle::SelfGuided as i32 => TourStyle::SelfGuided,
            n if n == TourStyle::Guided as i32 => TourStyle::Guided,
            _ => unreachable!(),
        },
        rentals,
    }))
}

async fn get_tour_rentals(pool: &PgPool, tour_id: OfferingId) -> Result<Vec<TourRental>, Error> {
    query!(
        "SELECT id, name FROM tour_rental JOIN offering ON rental_id = id WHERE tour_id = $1",
        tour_id.0
    )
    .fetch_all(pool)
    .await
    .map_err(|e| Error::Unknown(e.into()))?
    .into_iter()
    .map(|r| {
        Ok(TourRental {
            id: OfferingId(r.id),
            name: r.name,
        })
    })
    .collect()
}

async fn get_rental_by_id(pool: &PgPool, rental_id: OfferingId) -> Result<Option<Rental>, Error> {
    query!(
        r#"SELECT vendor_id, name FROM offering JOIN rental USING (id) WHERE id = $1"#,
        rental_id.0
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| Error::Unknown(e.into()))?
    .map(|r| {
        Ok(Rental {
            id: rental_id,
            vendor: r.vendor_id.into(),
            name: r.name,
        })
    })
    .transpose()
}

async fn save_tour(pool: &PgPool, tour: &Tour) -> Result<Tour, Error> {
    let mut txn = pool.begin().await.map_err(|e| Error::Unknown(e.into()))?;

    query!(
        "INSERT INTO offering (id, vendor_id, name) VALUES ($1, $2, $3) RETURNING id, vendor_id, name",
        tour.id.0,
        tour.vendor.inner(),
        tour.name
    )
        .fetch_one(&mut *txn)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

    query!(
        "INSERT INTO tour (id, style) VALUES ($1, $2) RETURNING id",
        tour.id.0,
        tour.style as i32
    )
    .fetch_one(&mut *txn)
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

    Ok(tour.clone())
}

async fn save_rental(pool: &PgPool, rental: &Rental) -> Result<Rental, Error> {
    let mut txn = pool.begin().await.map_err(|e| Error::Unknown(e.into()))?;

    let res = query!(
        "INSERT INTO offering (id, vendor_id, name) VALUES ($1, $2, $3) RETURNING id, vendor_id, name",
        rental.id.0,
        rental.vendor.inner(),
        rental.name
    )
    .fetch_one(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    query!(
        "INSERT INTO rental (id) VALUES ($1) RETURNING id",
        rental.id.0
    )
    .fetch_one(&mut *txn)
    .await
    .map_err(|e| Error::Unknown(e.into()))?;

    Ok(Rental {
        id: OfferingId(res.id),
        vendor: res.vendor_id.into(),
        name: res.name,
    })
}
