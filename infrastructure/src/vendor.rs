use application::vendor::ports::VendorRepository;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PgVendorRepository {
    pool: PgPool,
}

impl PgVendorRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl VendorRepository for PgVendorRepository {}
