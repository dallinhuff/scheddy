pub type CustomerId = crate::Id<Customer>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhoneNumber(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Customer {
    id: CustomerId,
    name: String,
    email: EmailAddress,
    phone_number: PhoneNumber,
}

impl Customer {
    pub fn new(
        id: CustomerId,
        name: impl Into<String>,
        email: EmailAddress,
        phone_number: PhoneNumber,
    ) -> Self {
        Customer {
            id,
            name: name.into(),
            email,
            phone_number,
        }
    }

    pub fn id(&self) -> CustomerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }
}
