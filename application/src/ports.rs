use domain::waiver::{Waiver, WaiverId};
use std::future::Future;

pub trait WaiverRepository {
    /// Fetches a [Waiver] from persistent storage by id.
    fn get_by_id(&self, id: WaiverId) -> impl Future<Output = Result<Option<Waiver>, String>>;

    /// Saves a [Waiver] to persistent storage.
    fn save(&self, waiver: Waiver) -> impl Future<Output = Result<Waiver, String>>;

    /// Deletes a [Waiver] from persistent storage.
    fn delete(&self, waiver: Waiver) -> impl Future<Output = Result<(), String>>;
}
