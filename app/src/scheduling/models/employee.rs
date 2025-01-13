/// A unique identifier for an employee.
pub type EmployeeId = crate::Id<Employee>;

/// The roles an employee may have when scheduled to work a shift.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Manager,
    Office,
    Guide,
}

/// A person who may work during a shift in some role/capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    id: EmployeeId,
    name: String,
    roles: std::collections::HashSet<Role>,
}

impl Employee {
    /// Create a new Employee with a newly generated Id.
    pub fn new(name: impl Into<String>, roles: impl IntoIterator<Item = Role>) -> Self {
        Self::existing(EmployeeId::new(), name, roles)
    }

    pub fn existing(
        id: EmployeeId,
        name: impl Into<String>,
        roles: impl IntoIterator<Item = Role>,
    ) -> Self {
        Employee {
            id,
            name: name.into(),
            roles: roles.into_iter().collect(),
        }
    }

    pub fn id(&self) -> EmployeeId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role)
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role);
    }

    pub fn add_roles(&mut self, roles: impl IntoIterator<Item = Role>) {
        self.roles.extend(roles);
    }

    pub fn remove_role(&mut self, role: Role) {
        self.roles.remove(&role);
    }

    pub fn remove_roles(&mut self, roles: impl IntoIterator<Item = Role>) {
        for role in roles {
            self.remove_role(role);
        }
    }
}
