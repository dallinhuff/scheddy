/// A release of liability to be signed by [Participant]s before participating
/// in a [Trip].
///
/// [Participant]: crate::participant::Participant
/// [Trip]: crate::trip::Trip
#[derive(Debug, Clone)]
pub struct Waiver {
    pub id: WaiverId,
    pub content: String
}

/// A unique identifier for a [Waiver].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaiverId(pub uuid::Uuid);

impl WaiverId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}