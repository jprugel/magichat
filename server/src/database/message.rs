use crate::User;

pub struct CreateMessageRequest {
    pub user: User,
    pub content: String,
    pub channel_id: String,
}

pub struct CreateMessageError(String);

pub struct ReadMessageRequest {
    pub id: String,
}

pub struct ReadMessageError(String);

pub struct UpdateMessageRequest {
    pub id: String,
    pub content: String,
}

pub struct UpdateMessageError(String);

pub struct DeleteMessageRequest {
    pub id: String,
}

pub struct DeleteMessageError(String);
