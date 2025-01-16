use crate::booking::models::{Customer, Equipment, Participant, ParticipantId, Trip};
use std::collections::HashMap;

pub type BookingId = crate::Id<Booking>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    id: BookingId,
    customer: Customer,
    trip: Trip,
    participants: HashMap<ParticipantId, Participant>,
    rentals: HashMap<Equipment, u16>,
}

impl Booking {
    pub fn new(
        id: BookingId,
        customer: Customer,
        trip: Trip,
        participants: impl IntoIterator<Item = Participant>,
        rentals: impl IntoIterator<Item = (Equipment, u16)>,
    ) -> Self {
        Booking {
            id,
            customer,
            trip,
            participants: participants.into_iter().map(|p| (p.id(), p)).collect(),
            rentals: rentals.into_iter().collect(),
        }
    }

    pub fn id(&self) -> BookingId {
        self.id
    }

    pub fn customer(&self) -> &Customer {
        &self.customer
    }

    pub fn trip(&self) -> &Trip {
        &self.trip
    }

    pub fn participants(&self) -> impl Iterator<Item = &Participant> {
        self.participants.values()
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.insert(participant.id(), participant);
    }

    pub fn remove_participant(&mut self, participant: &Participant) {
        self.participants.remove(&participant.id());
    }

    pub fn rent(&mut self, equipment: Equipment, quantity: u16) -> Result<(), ()> {
        self.rentals
            .entry(equipment)
            .and_modify(|q| *q += quantity)
            .or_insert(quantity);

        Ok(())
    }

    pub fn cancel_rental(&mut self, equipment: Equipment) -> Result<(), ()> {
        self.rentals.remove(&equipment);

        Ok(())
    }
}
