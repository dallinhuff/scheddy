use crate::customer::CustomerId;
use crate::participant::ParticipantId;
use crate::rental::{EquipmentKind, RentalId};
use crate::trip::{TripId, TripKind};
use std::collections::HashMap;

/// A reservation for a party of [Participant]s to participate in a [Trip].
///
/// [Participant]: crate::participant::Participant
/// [Trip]: crate::trip::Trip
#[derive(Debug, Clone)]
pub struct Booking {
    pub id: BookingId,
    pub customer: BookingCustomer,
    pub trip: BookingTrip,
    pub participants: Vec<BookingParticipant>,
    pub rentals: HashMap<BookingRental, u16>,
}

/// A unique identifier for a [Booking].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingId(pub uuid::Uuid);

impl BookingId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

/// A [Customer] who made a [Booking] with the relevant/pertinent customer info.
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingRental {
    pub id: RentalId,
    pub equipment: EquipmentKind,
    pub quantity: u16,
}
