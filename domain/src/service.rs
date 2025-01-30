use crate::rental::Rental;
use crate::trip::Trip;

#[derive(Debug, Clone)]
pub enum Service {
    Tour(Trip),
    Rental(Rental),
}
