pub mod service {
    use crate::vendor::ports::VendorRepository;

    pub trait VendorService {}

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
    pub trait VendorRepository {}
}
