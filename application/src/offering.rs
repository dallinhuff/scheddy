pub mod service {
    use crate::offering::ports::OfferingRepository;

    pub trait OfferingService {}

    pub struct OfferingServiceLive<T: OfferingRepository> {
        repo: T,
    }

    impl<T: OfferingRepository> OfferingServiceLive<T> {
        pub fn new(repo: T) -> OfferingServiceLive<T> {
            Self { repo }
        }
    }

    impl<T: OfferingRepository> OfferingService for OfferingServiceLive<T> {}
}

pub mod ports {
    use domain::offering::{Offering, OfferingId};
    use domain::vendor::VendorId;
    use std::future::Future;

    pub trait OfferingRepository {
        fn get_offering_by_id(
            &self,
            id: OfferingId,
        ) -> impl Future<Output = Result<Option<Offering>, RepositoryError>>;

        fn get_offerings_by_vendor(
            &self,
            vendor_id: VendorId,
        ) -> impl Future<Output = Result<Vec<Offering>, RepositoryError>>;

        fn get_tours_by_vendor(
            &self,
            vendor_id: VendorId,
        ) -> impl Future<Output = Result<Vec<Offering>, RepositoryError>>;

        fn get_rentals_by_vendor(
            &self,
            vendor_id: VendorId,
        ) -> impl Future<Output = Result<Vec<Offering>, RepositoryError>>;

        fn save(
            &self,
            offering: Offering,
        ) -> impl Future<Output = Result<Offering, RepositoryError>>;

        fn delete(&self, id: OfferingId) -> impl Future<Output = Result<(), RepositoryError>>;
    }

    pub enum RepositoryError {}
}
