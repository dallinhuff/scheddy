use crate::offering::ports::repository;
use crate::offering::ports::repository::OfferingRepository;
use domain::offering::{Offering, OfferingId};
use domain::vendor::VendorId;

/// A service for managing & orchestrating CRUD operations for [Offering]s.
#[trait_variant::make(Send)]
pub trait OfferingService {
    async fn get_offering_by_id(&self, id: OfferingId) -> Result<Option<Offering>, Error>;

    async fn get_offerings_by_vendor(&self, id: VendorId) -> Result<Vec<Offering>, Error>;
}

/// An error that may occur when calling a fallible method in [OfferingService].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An unknown/downstream error occurred.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<repository::Error> for Error {
    fn from(err: repository::Error) -> Self {
        Self::Unknown(err.into())
    }
}

/// An implementation of [OfferingService] that uses an
/// [OfferingRepository] to read & save persistent data.
#[derive(Clone)]
pub struct OfferingServiceLive<T: OfferingRepository> {
    repo: T,
}

impl<T: OfferingRepository> OfferingServiceLive<T> {
    pub fn new(repo: T) -> OfferingServiceLive<T> {
        Self { repo }
    }
}

impl<T: OfferingRepository> OfferingService for OfferingServiceLive<T> {
    async fn get_offering_by_id(&self, id: OfferingId) -> Result<Option<Offering>, Error> {
        self.repo.get_offering_by_id(id).await.map_err(Into::into)
    }

    async fn get_offerings_by_vendor(&self, id: VendorId) -> Result<Vec<Offering>, Error> {
        self.repo
            .get_offerings_by_vendor(id)
            .await
            .map_err(Into::into)
    }
}
