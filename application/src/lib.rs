//! [application][crate] defines the application use cases for interacting with
//! the [domain] and port traits for adapters to implement. It orchestrates all
//! features the application provides while remaining generic over any concrete
//! infrastructure decisions/implementations.
//!
//! [domain]: ../domain

pub mod ports;