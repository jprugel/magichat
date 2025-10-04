use crate::filetype::{FileType, get_file_type};
use crate::*;
use axum::Json;
use axum::http::StatusCode;
use protocol::{Channel, Icon, Server};

/// Handles the request for server information.
///
/// # OpenAPI
/// path: /info
/// method: GET
/// response: (GuildInfo, StatusCode)
pub async fn handler(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let icon_path = state.config.info.icon;
    let filetype = get_file_type(&icon_path);
    let icon = match filetype {
        FileType::Png => Icon::Svg(icon_path),
        _ => Icon::Default,
    };

    let server_info = Server {
        url: state.config.server.to_addr(),
        name: state.config.info.server_name,
        icon,
        channel_list: state
            .config
            .info
            .channels
            .into_iter()
            .map(Channel::from_name)
            .collect(),
    };

    (StatusCode::ACCEPTED, Json(server_info))
}
