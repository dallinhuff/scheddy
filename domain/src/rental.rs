pub struct Rental {
    id: RentalId,
    equipment: EquipmentKind,
    quantity: u16
}

pub struct RentalId(uuid::Uuid);

pub enum EquipmentKind {
    Tube(TubeKind),
    Raft(RaftKind),
    Kayak(KayakKind),
}

pub enum TubeKind {}
pub enum RaftKind {}
pub enum KayakKind {}