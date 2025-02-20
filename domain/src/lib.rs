//! [domain][crate] provides the core business types, logic, and rules of the
//! application.
//!
//! It is agnostic of any outside ports/infrastructure, deferring such
//! orchestration & interaction to an outer layer.

#![warn(clippy::pedantic)]

pub mod offering;
pub mod user;
pub mod vendor;

pub use user::{User, UserId};
pub use vendor::{Vendor, VendorId};
