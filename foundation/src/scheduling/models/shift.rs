use chrono::{NaiveDateTime, TimeDelta};

pub type ShiftId = crate::Id<Shift>;

pub struct Shift {
    id: ShiftId,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Shift {
    pub fn id(&self) -> ShiftId {
        self.id
    }

    pub fn start(&self) -> NaiveDateTime {
        self.start
    }

    pub fn end(&self) -> NaiveDateTime {
        self.end
    }

    pub fn duration(&self) -> TimeDelta {
        self.end - self.start
    }
}
