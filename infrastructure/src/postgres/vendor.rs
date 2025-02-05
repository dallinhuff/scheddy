use crate::postgres::Postgres;
use application::vendor::ports::repository::{Error, VendorRepository};
use domain::vendor::{Vendor, VendorId};
use sqlx::{query, query_as, FromRow};
use uuid::Uuid;

impl VendorRepository for Postgres {
    async fn get_by_id(&self, id: VendorId) -> Result<Option<Vendor>, Error> {
        query_as!(VendorDto, "select id, name from vendor where id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Unknown(e.into()))
            .map(|v| v.map(Vendor::from))
    }

    async fn create(&self, vendor: Vendor) -> Result<Vendor, Error> {
        let VendorDto { id, name } = VendorDto::from(vendor);
        query_as!(
            VendorDto,
            "insert into vendor (id, name) values ($1, $2) returning id, name",
            id,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))
        .map(Vendor::from)
    }

    async fn update(&self, vendor: Vendor) -> Result<Vendor, Error> {
        let VendorDto { id, name } = VendorDto::from(vendor);
        query_as!(
            VendorDto,
            "update vendor set name = $1 where id = $2 returning id, name",
            name,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))
        .map(Vendor::from)
    }

    async fn delete(&self, id: VendorId) -> Result<(), Error> {
        query!("delete from vendor where id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Unknown(e.into()))
            .map(|_| ())
    }
}

#[derive(FromRow)]
struct VendorDto {
    id: Uuid,
    name: String,
}

impl From<VendorDto> for Vendor {
    fn from(dto: VendorDto) -> Self {
        Vendor {
            id: VendorId(dto.id),
            name: dto.name,
        }
    }
}

impl From<Vendor> for VendorDto {
    fn from(v: Vendor) -> Self {
        VendorDto {
            id: v.id.0,
            name: v.name,
        }
    }
}
