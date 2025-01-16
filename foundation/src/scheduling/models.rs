//! `models` contains the domain entities/value-objects/aggregates for expressing
//! the data & business logic for scheduling.

mod employee;
pub use employee::*;

mod shift;
pub use shift::*;

mod trip;
pub use trip::*;
