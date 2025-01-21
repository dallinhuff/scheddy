#[derive(Debug, Clone)]
pub struct Customer {
    pub id: CustomerId,
    pub name: CustomerName,
    pub email: CustomerEmail,
    pub phone_number: CustomerPhoneNumber,
    pub preferences: CustomerPreferences,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerId(uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerName(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerEmail(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerPhoneNumber(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerPreferences {
    pub preferred_contact_method: ContactMethod,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContactMethod {
    Email,
    Text,
    Call,
}
