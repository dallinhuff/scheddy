pub struct Waiver {
    pub id: WaiverId,
    pub content: String
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WaiverId(i32);

impl WaiverId {
    pub const fn new(id: i32) -> Self {
        WaiverId(id)
    }

    pub const fn null() -> Self {
        WaiverId(0)
    }

    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

impl From<i32> for WaiverId {
    fn from(id: i32) -> Self {
        Self::new(id)
    }
}