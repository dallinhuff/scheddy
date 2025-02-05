pub mod repository {
    use domain::vendor::{Vendor, VendorId};

    #[trait_variant::make(Send)]
    pub trait VendorRepository: Sync {
        async fn get_by_id(&self, id: VendorId) -> Result<Option<Vendor>, Error>;
        async fn create(&self, vendor: Vendor) -> Result<Vendor, Error>;
        async fn update(&self, vendor: Vendor) -> Result<Vendor, Error>;
        async fn delete(&self, id: VendorId) -> Result<(), Error>;
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    }
}
