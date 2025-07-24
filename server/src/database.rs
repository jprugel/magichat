use sqlx::{postgres, Error, Pool, Row};
use sqlx::pool::PoolOptions;
use sqlx::postgres::{PgPoolOptions, PgRow};
use protocol::{Icon, User};
use protocol::user::{CreateUserError, CreateUserRequest, UserRepository, Username, UserId, ReadUserRequest, ReadUserError, UpdateUserRequest, DeleteUserRequest, DeleteUserError, UpdateUserError};
use sqlx::FromRow;
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

        Ok(Database {
            pool,
        })
    }
}

impl UserRepository for Database {
    async fn create_user(&self, request: &CreateUserRequest) -> Result<User, CreateUserError> {
        let CreateUserRequest { username, password } = request;
        let user_id = Uuid::new_v4();
        // I need to somehow create a user in the database.
        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (id, username, password, totp_verified, icon)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, username as "username: Username", password as "password: Password", totp_verified, icon as "icon: Icon"
            "#)
            .bind(UserId(user_id.to_string()))
            .bind(username)
            .bind(password)
            .bind(false)
            .bind(false)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| CreateUserError::new("Failed to create user".to_string()))
    }

    async fn read_user(&self, request: &ReadUserRequest) -> Result<Option<User>, ReadUserError> {
        let ReadUserRequest { user_id } = request;
        
        sqlx::query_as::<_, User>(
            r#"
                Select id, username as "username: Username", password as "password: Password", totp_verified, icon as "icon: Icon"
                FROM users where id = $1
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ReadUserError::new("Failed to create user"))
    }

    async fn update_user(&self, request: &UpdateUserRequest) -> Result<User, UpdateUserError> {
        let UpdateUserRequest { 
            user_id, 
            username, 
            password, 
            totp_verified, 
            icon 
        } = request;
        
        let existing_user = sqlx::query_as::<_, User>(
            r#"
                Select id, username as "username: Username", password as "password: Password", totp_verified, icon as "icon: Icon"
                FROM users where id = $1
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ReadUserError::new("Failed to create user"))?;
    }

    async fn delete_user(&self, request: &DeleteUserRequest) -> Result<User, DeleteUserError> {
        todo!()
    }
}