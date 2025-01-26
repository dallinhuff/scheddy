use uuid::Uuid;

/// A release of liability to be signed by [Participant]s before participating
/// in a [Trip].
///
/// [Participant]: crate::participant::Participant
/// [Trip]: crate::trip::Trip
#[derive(Debug, Clone)]
pub struct Waiver {
    pub id: WaiverId,
    pub content: String,
}

/// A unique identifier for a [Waiver].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaiverId(pub Uuid);

impl WaiverId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<Uuid> for WaiverId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<WaiverId> for Uuid {
    fn from(id: WaiverId) -> Self {
        id.0
    }
}
