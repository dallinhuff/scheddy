use crate::Id;

pub type TripId = Id<Trip>;

pub use crate::scheduling::models::TripKind;
pub use crate::scheduling::models::TripLocation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Trip {
    id: TripId,
    kind: TripKind,
    location: TripLocation,
    num_participants: u16,
    capacity: u16,
}

impl Trip {
    pub fn id(&self) -> TripId {
        self.id
    }
}
