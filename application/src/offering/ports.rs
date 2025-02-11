pub mod repository {
    use domain::offering::{Offering, OfferingId, Rental, Tour};
    use domain::vendor::VendorId;

    /// A repository for reading & persisting [Offering] data.
    #[trait_variant::make(Send)]
    pub trait OfferingRepository: Sync {
        async fn get_offering_by_id(&self, id: OfferingId) -> Result<Option<Offering>, Error>;

        async fn get_offerings_by_vendor(&self, id: VendorId) -> Result<Vec<Offering>, Error>;

        async fn get_tours_by_vendor(&self, id: VendorId) -> Result<Vec<Tour>, Error>;

        async fn get_rentals_by_vendor(&self, id: VendorId) -> Result<Vec<Rental>, Error>;

        async fn save(&self, offering: Offering) -> Result<Offering, Error>;

        async fn delete(&self, id: OfferingId) -> Result<(), Error>;
    }

    /// An error that may occur while calling a fallible method in [`OfferingRepository`].
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    }
}
