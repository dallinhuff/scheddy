use crate::waiver::WaiverId;

#[derive(Debug, Clone)]
pub struct Participant {
    pub id: ParticipantId,
    pub name: ParticipantName,
    pub dob: ParticipantDateOfBirth,
    pub waiver: Option<ParticipantWaiver>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticipantId(pub uuid::Uuid);

impl ParticipantId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticipantName(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticipantDateOfBirth(chrono::NaiveDate);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticipantWaiver(WaiverId, chrono::NaiveDate);
