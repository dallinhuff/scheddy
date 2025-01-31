use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password: Password,
    pub email: EmailAddress,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Password(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress(pub String);
