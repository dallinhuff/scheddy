use crate::customer::CustomerId;
use crate::participant::ParticipantId;
use crate::trip::{TripId, TripKind};

#[derive(Debug, Clone)]
pub struct Booking {
    pub id: BookingId,
    pub customer: BookingCustomer,
    pub trip: BookingTrip,
    pub participants: Vec<BookingParticipant>,
    pub rentals: BookingRentals,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BookingId(uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingCustomer {
    pub id: CustomerId,
}

#[derive(Debug, Clone)]
pub struct BookingTrip {
    pub id: TripId,
    pub kind: TripKind,
    pub start_time: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingParticipant {
    pub id: ParticipantId,
    pub waiver_signed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookingRentals;
