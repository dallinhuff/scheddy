use crate::postgres::Postgres;
use application::vendor::ports::repository::{Error, VendorRepository};
use domain::vendor::{Vendor, VendorId};
use sqlx::{FromRow, query, query_as};
use uuid::Uuid;

impl VendorRepository for Postgres {
    async fn get_by_id(&self, id: VendorId) -> Result<Option<Vendor>, Error> {
        query_as!(
            VendorDto,
            "SELECT * FROM vendor WHERE vendor_id = $1",
            id.inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))
        .map(|v| v.map(Vendor::from))
    }

    async fn create(&self, vendor: Vendor) -> Result<Vendor, Error> {
        let VendorDto { vendor_id, name } = VendorDto::from(vendor);
        query_as!(
            VendorDto,
            "INSERT INTO vendor (vendor_id, name) VALUES ($1, $2) RETURNING vendor_id, name",
            vendor_id,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))
        .map(Vendor::from)
    }

    async fn update(&self, vendor: Vendor) -> Result<Vendor, Error> {
        let VendorDto { vendor_id, name } = VendorDto::from(vendor);
        query_as!(
            VendorDto,
            "UPDATE vendor SET name = $1 WHERE vendor_id = $2 RETURNING vendor_id, name",
            name,
            vendor_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))
        .map(Vendor::from)
    }

    async fn delete(&self, id: VendorId) -> Result<(), Error> {
        query!("DELETE FROM vendor WHERE vendor_id = $1", id.inner())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Unknown(e.into()))
            .map(|_| ())
    }
}

#[derive(FromRow)]
struct VendorDto {
    vendor_id: Uuid,
    name: String,
}

impl From<VendorDto> for Vendor {
    fn from(dto: VendorDto) -> Self {
        Vendor {
            id: dto.vendor_id.into(),
            name: dto.name,
        }
    }
}

impl From<Vendor> for VendorDto {
    fn from(v: Vendor) -> Self {
        VendorDto {
            vendor_id: v.id.into(),
            name: v.name,
        }
    }
}
