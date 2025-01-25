use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Trip {
    pub id: TripId,
    pub kind: TripKind,
    pub location: TripLocation,
    pub max_capacity: i32,
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TripId(pub uuid::Uuid);

impl TripId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TripKind {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TripLocation {
    Provo,
    Weber,
}
