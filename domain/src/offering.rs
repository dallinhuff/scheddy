use crate::vendor::VendorId;
use uuid::Uuid;

/// A general offering/service provided by a [Vendor].
///
/// [Vendor]: crate::vendor::Vendor
#[derive(Debug, Clone)]
pub struct Offering {
    pub id: OfferingId,
    pub vendor: OfferingVendor,
    pub name: String,
    pub description: String,
    pub kind: OfferingKind,
}

/// A unique identifier for an [Offering].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OfferingId(pub Uuid);

/// A pertinent view of a [Vendor] from an [Offering]'s perspective.
#[derive(Debug, Clone)]
pub struct OfferingVendor {
    pub id: VendorId,
}

/// A variant of [Offering]
/// i.e, the kinds & specific details that an [Offering] may have & entail.
#[derive(Debug, Clone, PartialEq)]
pub enum OfferingKind {
    Tour {},
    Rental {},
}
