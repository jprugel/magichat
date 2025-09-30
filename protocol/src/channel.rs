struct Channel {
    id: Uuid,
    name: String,
    description: String,
}

struct CreateChannelRequest(String);
struct CreateChannelError(String);

struct ReadChannelRequest(Uuid);
struct ReadChannelError(String);

struct UpdateChannelRequest(Uuid, String, String);
struct UpdateChannelError(String);

struct DeleteChannelRequest(Uuid);
struct DeleteChannelError(String);

pub trait ChannelRepository: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        request: &CreateChannelRequest,
    ) -> impl Future<Output = Result<Channel, CreateChannelError>> + Send;

    fn read_user(
        &self,
        request: &ReadChannelRequest,
    ) -> impl Future<Output = Result<Option<Channel>, ReadChannelError>> + Send;

    fn update_user(
        &self,
        request: &UpdateChannelRequest,
    ) -> impl Future<Output = Result<Option<Channel>, UpdateChannelError>> + Send;

    fn delete_user(
        &self,
        request: &DeleteChannelRequest,
    ) -> impl Future<Output = Result<Option<Channel>, DeleteChannelError>> + Send;
}
