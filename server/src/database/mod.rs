pub mod message;
pub mod user;

pub use protocol::user::UserRepository;
use protocol::{Icon, User};
use sqlx::pool::PoolOptions;
use sqlx::{Error, Pool, Row, postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    pool: Pool<sqlx::Postgres>,
}

impl Database {
    pub(crate) fn builder() -> DatabaseBuilder {
        DatabaseBuilder::default()
    }
}

#[derive(Default)]
pub struct DatabaseBuilder {
    connections: u32,
    url: String,
}

impl DatabaseBuilder {
    pub(crate) fn pool(mut self, connections: u32) -> DatabaseBuilder {
        self.connections = connections;
        self
    }

    pub(crate) fn url(mut self, url: &str) -> DatabaseBuilder {
        self.url = url.to_string();
        self
    }

    pub(crate) async fn build(self) -> Result<Database, sqlx::Error> {
        let pool = PoolOptions::new()
            .max_connections(self.connections)
            .connect(&self.url)
            .await?;

        Ok(Database { pool })
    }
}
