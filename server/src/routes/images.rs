use axum::{
    extract::{Multipart, Path},
    http::{header, StatusCode, HeaderMap, HeaderValue},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tokio::fs;
use uuid::Uuid;
use std::{path::PathBuf, net::SocketAddr};

#[axum::debug_handler]
pub(crate) async fn upload_handler(mut multipart: Multipart) -> Json<String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();
        let filename = format!("images/{}.png", Uuid::new_v4());

        fs::create_dir_all("images").await.unwrap();
        fs::write(&filename, &data).await.unwrap();

        return Json(format!("Saved {} as {}", name, filename));
    }

    Json("No file found".into())
}

#[axum::debug_handler]
pub(crate) async fn image_handler(Path(image_name): Path<String>) -> impl IntoResponse {
    // Build the path on disk
    let filepath = PathBuf::from("./server/assets/images/").join(format!("{}.png", image_name));

    match fs::read(&filepath).await {
        Ok(image_data) => {
            // Build headers
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));

            // Return image bytes with headers
            (headers, image_data).into_response()
        }
        Err(_) => {
            // File not found or other error
            (StatusCode::NOT_FOUND, "Image not found").into_response()
        }
    }
}
