//! [domain][crate] provides the core business types, logic, and rules of the
//! application.
//!
//! It is agnostic of any outside ports/infrastructure, deferring such
//! orchestration & interaction to an outer layer.

pub mod booking;
pub mod customer;
pub mod employee;
pub mod participant;
pub mod rental;
pub mod schedule;
pub mod trip;
pub mod waiver;
