use uuid::Uuid;

/// A [Vendor] provides one or more [Offering]s that a
/// [Customer] can schedule [Booking]s for.
///
/// [Offering]: crate::offering::Offering
/// [Customer]: crate::customer::Customer
/// [Booking]: crate::booking::Booking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vendor {
    pub id: VendorId,
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct VendorId(Uuid);

impl VendorId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl Default for VendorId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for VendorId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<VendorId> for Uuid {
    fn from(vendor_id: VendorId) -> Self {
        vendor_id.0
    }
}
