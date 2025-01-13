//! `ports` defines the ways outside systems may interact with the scheduling
//! domain (and vice versa).

use crate::scheduling::models::employee::{Employee, EmployeeId};
use crate::scheduling::models::shift::{Shift, ShiftId};
use std::future::Future;
use std::sync::Arc;

use super::models::employee::Role;

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
