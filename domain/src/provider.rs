/// A [Provider] provides one or more [Service]s that a
/// [Customer] can schedule [Booking]s for.
///
/// [Service]: crate::service::Service
/// [Customer]: crate::customer::Customer
/// [Booking]: crate::booking::Booking
#[derive(Debug, Clone)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProviderId(pub uuid::Uuid);

impl ProviderId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}
