use sqlx::{SqlitePool, sqlite::SqliteRow};
use redb::{Database, TableDefinition};
use crate::error::Error;
use crate::core::auth::*;
use crate::models::*;

pub struct HybridStore {
    sqlite: SqlitePool,
    redb: Database,
    encryption: Arc<crate::core::encryption::ContentEncryption>,
}

impl HybridStore {
    pub async fn new(config: &crate::app_state::AppState) -> Result<Self, Error> {
        let sqlite = SqlitePool::connect(&config.database_url).await?;
        let redb = Database::create("ephemeral.redb")?;
        let encryption = Arc::new(crate::core::encryption::ContentEncryption::new(
            config.encryption_key.as_deref()
        )?);

        Ok(Self { sqlite, redb, encryption })
    }
}