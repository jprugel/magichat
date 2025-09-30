use crate::User;
use crate::database::Database;
pub use protocol::user::UserRepository;
use protocol::user::{
    CreateUserError, CreateUserRequest, DeleteUserError, DeleteUserRequest, ReadUserError,
    ReadUserRequest, UpdateUserError, UpdateUserRequest, UserId,
};
use uuid::Uuid;

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
            .map_err(|e| ReadUserError::new("Failed to create user".to_string()))
    }

    async fn update_user(
        &self,
        request: &UpdateUserRequest,
    ) -> Result<Option<User>, UpdateUserError> {
        let UpdateUserRequest {
            user_id,
            username,
            password,
            totp_verified,
            icon,
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
            .map_err(|e| UpdateUserError::new("Failed to create user".to_string()));

        existing_user
    }

    async fn delete_user(
        &self,
        request: &DeleteUserRequest,
    ) -> Result<Option<User>, DeleteUserError> {
        let DeleteUserRequest { user_id } = request;

        let user = sqlx::query_as(
            r#"
                DELETE FROM users
                WHERE id = $1
                RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| DeleteUserError::new("Failed to delete user.".to_string()));

        user
    }
}
