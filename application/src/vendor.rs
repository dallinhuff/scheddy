pub mod service {
    use crate::vendor::ports::VendorRepository;

    /// A [VendorService] manages & orchestrates the application use-cases for
    /// interacting with [Vendor]s.
    pub trait VendorService {}

    /// An implementation of [VendorService] that uses a [VendorRepository]
    /// to access & persist [Vendor] data.
    #[derive(Debug, Clone)]
    pub struct VendorServiceLive<R: VendorRepository> {
        repo: R,
    }

    impl<R: VendorRepository> VendorServiceLive<R> {
        pub const fn new(repo: R) -> VendorServiceLive<R> {
            Self { repo }
        }
    }

    impl<R: VendorRepository> VendorService for VendorServiceLive<R> {}
}

pub mod ports {
    use domain::vendor::{Vendor, VendorId};
    use std::future::Future;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Error {}

    /// A [VendorRepository] can asynchronously access & persist [Vendor] data.
    pub trait VendorRepository {
        fn get_by_id(&self, id: VendorId) -> impl Future<Output = Result<Option<Vendor>, Error>>;
        fn create(&self, vendor: Vendor) -> impl Future<Output = Result<Vendor, Error>>;
        fn update(&self, vendor: Vendor) -> impl Future<Output = Result<Vendor, Error>>;
        fn delete(&self, id: VendorId) -> impl Future<Output = Result<(), Error>>;
    }
}
