use crate::vendor::VendorId;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// A good or service that a [Vendor] may offer to [Customer]s.
///
/// [Vendor]: crate::vendor::Vendor
/// [Customer]: crate::customer::Customer
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
pub enum Offering {
    Rental(Rental),
    Tour(Tour),
}

/// A unique identifier for an [Offering].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct OfferingId(pub Uuid);

/// An [Offering] for an equipment rental.
/// It may be either a child of a [Tour] or an offering in its own right.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rental {
    pub id: OfferingId,
    pub vendor: VendorId,
    pub name: String,
}

/// An [Offering] for a scheduled experience.
/// It may be guided or self-guided.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tour {
    pub id: OfferingId,
    pub vendor: VendorId,
    pub name: String,
    pub style: TourStyle,
    pub rentals: Vec<TourRental>,
}

/// The style/modality of a [Tour].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(variant_identifier))]
pub enum TourStyle {
    Guided = 1,
    SelfGuided = 2,
}

impl Display for TourStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TourStyle::Guided => write!(f, "Guided"),
            TourStyle::SelfGuided => write!(f, "Self-Guided"),
        }
    }
}

/// A child-rental of a [Tour], backed by an underlying [Rental].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TourRental {
    pub id: OfferingId,
    pub name: String,
}
