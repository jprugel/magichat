use std::ops::Deref;
use serde::{Deserialize, Serialize};
use sqlx::{ FromRow, Type};
use crate::{Icon, User};

#[derive(Clone, Debug, Serialize, Deserialize, Default, FromRow, Type)]
#[sqlx(transparent)]
pub struct Username(String);
pub struct UsernameError(String);

impl Username {
    fn new(username: String) -> Result<Self, UsernameError> {
        if username.is_empty() {
            return Err(UsernameError("Username cannot be empty".to_string()));
        }

        Ok(Username(username))
    }
}

impl From<String> for Username {
    fn from(raw: String) -> Self {
        Self(raw)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Default, FromRow, Type)]
#[sqlx(transparent)]
pub struct Password(String);
pub struct PasswordError(String);

impl Password {
    fn new(password: String) -> Result<Self, PasswordError> {
        if password.is_empty() {
            return Err(PasswordError("Password cannot be empty".to_string()));
        }

        Ok(Password(password))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Password {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Password {
    fn from(raw: String) -> Self {
        Self(raw)
    }
}

pub struct CreateUserRequest {
    pub username: Username,
    pub password: Password,
}

pub struct ReadUserRequest {
    pub user_id: UserId,
}

pub struct UpdateUserRequest {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub password: Option<Password>,
    pub totp_verified: Option<bool>,
    pub icon: Option<Icon>
}

pub struct DeleteUserRequest {
    pub user_id: UserId,
}

pub struct CreateUserError(String);
pub struct ReadUserError(String);

pub struct UpdateUserError(String);
pub struct DeleteUserError(String);

impl CreateUserError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl ReadUserError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl UpdateUserError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl DeleteUserError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

pub trait UserRepository: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        request: &CreateUserRequest
    ) -> impl Future<Output = Result<User, CreateUserError>> + Send;
    
    fn read_user(
        &self,
        request: &ReadUserRequest
    ) -> impl Future<Output = Result<Option<User>, ReadUserError>> + Send;
    
    fn update_user(
        &self,
        request: &UpdateUserRequest
    ) -> impl Future<Output = Result<Option<User>, UpdateUserError>> + Send;
    
    fn delete_user(
        &self,
        request: &DeleteUserRequest
    ) -> impl Future<Output = Result<Option<User>, DeleteUserError>> + Send;
}

#[derive(FromRow, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct UserId(pub String);