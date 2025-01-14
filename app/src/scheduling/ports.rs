//! `ports` defines the ways outside systems may interact with the scheduling
//! domain (and vice versa).

use crate::scheduling::models::{Employee, EmployeeId, Role};
use crate::scheduling::models::{Shift, ShiftId};
use std::future::Future;
use std::sync::Arc;

pub trait Repository {
    fn get_employee(
        &self,
        id: EmployeeId,
    ) -> impl Future<Output = Result<Option<Employee>, ()>> + Send;

    fn save_employee(
        &self,
        employee: &Employee,
    ) -> impl Future<Output = Result<Employee, ()>> + Send;

    fn get_shift(&self, id: ShiftId) -> impl Future<Output = Result<Option<Shift>, ()>> + Send;

    fn save_shift(&self, shift: &Shift) -> impl Future<Output = Result<Shift, ()>> + Send;

    fn get_available_employees(
        &self,
        shift: ShiftId,
        role: Option<Role>,
    ) -> impl Future<Output = Result<Arc<[Employee]>, ()>> + Send;
}
