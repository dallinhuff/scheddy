use std::collections::HashSet;

/// A person who may be scheduled to work during a [Shift].
///
/// [Shift]: crate::schedule::Shift.
#[derive(Debug, Clone)]
pub struct Employee {
    pub id: EmployeeId,
    pub name: EmployeeName,
    pub email: EmployeeEmail,
    pub phone_number: EmployeePhoneNumber,
    pub roles: HashSet<EmployeeRole>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EmployeeId(pub uuid::Uuid);

impl EmployeeId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmployeeName(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmployeeEmail(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmployeePhoneNumber(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmployeeRole {
    Office,
    Shuttle,
    Guide,
    Medical,
    Catering,
}
