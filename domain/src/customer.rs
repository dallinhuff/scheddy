/// A person responsible for making a [Booking].
///
/// [Booking]: crate::booking::Booking
#[derive(Debug, Clone)]
pub struct Customer {
    pub id: CustomerId,
    pub name: CustomerName,
    pub email: CustomerEmail,
    pub phone_number: CustomerPhoneNumber,
    pub preferences: CustomerPreferences,
}

/// A unique identifier for a [Customer].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CustomerId(pub uuid::Uuid);

impl CustomerId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

/// The full/preferred name of a [Customer].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerName(String);

impl CustomerName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// An email address used as a point of contact for a [Customer].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerEmail(String);

impl CustomerEmail {
    pub fn new(email: impl Into<String>) -> Self {
        Self(email.into())
    }
}

/// A phone number used as a point of contact for a [Customer].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerPhoneNumber {
    pub phone_number: String,
    pub phone_type: PhoneType,
    pub sms: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhoneType {
    Home,
    Mobile,
    Work,
}

/// Configurable preferences a [Customer] may have.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerPreferences {
    pub preferred_contact_method: ContactMethod,
}

/// Ways a [Customer] may be contacted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContactMethod {
    Email,
    Text,
    Call,
}
