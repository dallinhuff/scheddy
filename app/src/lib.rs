//! The `app` crate contains the core/shared application types & logic
//! for the scheddy application. Modules are separated by domain concern,
//! with each module defining models, ports, and services for that domain.

pub mod booking;
pub mod scheduling;

// A general ID type for use in domain models.
// It takes a generic marker type to ensure that it can only be used as
// and identifier for models of that type.
//
// It implements Copy and is backed by a UUID v7.
// For better ergonomics, nodules wishing to use this
// type should expose a public type alias where T is concretized.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T>(uuid::Uuid, std::marker::PhantomData<T>);

impl<T> Id<T> {
    // Create a new unique Id.
    pub fn new() -> Self {
        Id(uuid::Uuid::now_v7(), std::marker::PhantomData)
    }

    // Get the inner/backing Uuid.
    pub fn inner(&self) -> uuid::Uuid {
        self.0
    }
}

impl<T> From<uuid::Uuid> for Id<T> {
    // Create an Id from an existing Uuid.
    fn from(raw: uuid::Uuid) -> Self {
        Id(raw, std::marker::PhantomData)
    }
}

impl<T> Copy for Id<T> {}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod id_tests {
    use super::Id;

    #[test]
    fn from_and_inner_are_inverses() {
        let given = uuid::Uuid::nil();
        let want = given;
        let got = Id::<i32>::from(given).inner();
        assert_eq!(want, got);
    }
}
