use chrono::NaiveDate;

pub type ParticipantId = crate::Id<Participant>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Participant {
    id: ParticipantId,
    name: String,
    dob: NaiveDate,
}

impl Participant {
    pub fn id(&self) -> ParticipantId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dob(&self) -> NaiveDate {
        self.dob
    }
}
