use crate::vendor::ports::{repository, repository::VendorRepository};
use domain::vendor::{Vendor, VendorId};

/// A [VendorService] manages & orchestrates the application use-cases for
/// interacting with [Vendor]s.
#[trait_variant::make(Send)]
pub trait VendorService {
    async fn get_by_id(&self, id: VendorId) -> Result<Option<Vendor>, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<repository::Error> for Error {
    fn from(error: repository::Error) -> Self {
        Self::Unknown(error.into())
    }
}

/// An implementation of [`VendorService`] that uses a [`VendorRepository`]
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

impl<R: VendorRepository> VendorService for VendorServiceLive<R> {
    async fn get_by_id(&self, id: VendorId) -> Result<Option<Vendor>, Error> {
        self.repo.get_by_id(id).await.map_err(Into::into)
    }
}
