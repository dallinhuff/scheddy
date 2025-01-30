use crate::booking::BookingId;

#[derive(Debug, Clone)]
pub struct Rental {
    pub id: RentalId,
    pub booking: BookingId,
    pub equipment: EquipmentKind,
    pub quantity: u16
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RentalId(pub uuid::Uuid);

impl RentalId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquipmentKind {
    Tube(TubeKind),
    Raft(RaftKind),
    Kayak(KayakKind),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TubeKind {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaftKind {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KayakKind {}