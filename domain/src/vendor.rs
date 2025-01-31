/// A [Vendor] provides one or more [Service]s that a
/// [Customer] can schedule [Booking]s for.
///
/// [Service]: crate::service::Service
/// [Customer]: crate::customer::Customer
/// [Booking]: crate::booking::Booking
#[derive(Debug, Clone)]
pub struct Vendor {
    pub id: VendorId,
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VendorId(pub uuid::Uuid);

impl VendorId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}
