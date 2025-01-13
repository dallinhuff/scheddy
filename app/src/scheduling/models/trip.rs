use chrono::{NaiveDateTime, TimeDelta};

pub type TripId = crate::Id<Trip>;

/// The possible kinds/categories that a Trip may be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TripKind {
    Tubing,
    Rafting { guided: bool },
    Kayaking { guided: bool },
}

/// The possible locations a Trip may depart from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TripLocation {
    Provo,
    Weber,
}

/// A scheduled activity of a certain kind, with an expected number of registered participants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trip {
    id: TripId,
    kind: TripKind,
    location: TripLocation,
    start_time: NaiveDateTime,
    num_participants: u16,
}

impl Trip {
    pub fn new(
        id: TripId,
        kind: TripKind,
        location: TripLocation,
        start_time: NaiveDateTime,
        num_participants: u16,
    ) -> Self {
        Trip {
            id,
            kind,
            location,
            start_time,
            num_participants,
        }
    }

    pub fn id(&self) -> TripId {
        self.id
    }

    pub fn kind(&self) -> TripKind {
        self.kind
    }

    pub fn location(&self) -> TripLocation {
        self.location
    }

    pub fn start_time(&self) -> NaiveDateTime {
        self.start_time
    }

    pub fn duration(&self) -> TimeDelta {
        match (self.kind, self.location) {
            (TripKind::Tubing, _) => TimeDelta::hours(2),
            (TripKind::Rafting { .. }, TripLocation::Provo) => TimeDelta::hours(3),
            (TripKind::Rafting { .. }, TripLocation::Weber) => TimeDelta::hours(4),
            (TripKind::Kayaking { .. }, TripLocation::Provo) => TimeDelta::hours(2),
            (TripKind::Kayaking { .. }, TripLocation::Weber) => TimeDelta::hours(3),
        }
    }

    pub fn end_time(&self) -> NaiveDateTime {
        self.start_time() + self.duration()
    }

    pub fn num_participants(&self) -> u16 {
        self.num_participants
    }
}
