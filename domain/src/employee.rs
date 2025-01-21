use std::collections::HashSet;

pub struct Employee {
    pub id: EmployeeId,
    pub name: EmployeeName,
    pub email: EmployeeEmail,
    pub phone_number: EmployeePhoneNumber,
    pub roles: HashSet<EmployeeRole>,
}

pub struct EmployeeId(uuid::Uuid);

pub struct EmployeeName(String);

pub struct EmployeeEmail(String);

pub struct EmployeePhoneNumber(String);

pub enum EmployeeRole {
    Office,
    Shuttle,
    Guide,
    Medical,
    Catering,
}
