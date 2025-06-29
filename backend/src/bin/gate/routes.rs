use axum::body::Body;
use axum::extract::State;
use axum::response::IntoResponse;
use backend::extractors::UserData;
use futures::StreamExt;
use hyper::StatusCode;
use image::imageops::FilterType;
use image::ImageReader;
use macros::routes;
use std::io::Cursor;
use webp::Encoder;

use crate::AppState;

pub struct Routes;

#[routes(prefix="/", state=AppState)]
impl Routes {
    #[route(method=post, path="/upload/img")]
    async fn upload(
        State(state): State<AppState>,
        user: UserData,
        body: Body,
    ) -> impl IntoResponse {
        let mut stream = body.into_data_stream();
        let mut buf = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            buf.extend_from_slice(&chunk);
            if buf.len() > 10_000_000 {
                return Err(StatusCode::PAYLOAD_TOO_LARGE);
            }
        }

        if let Some(kind) = infer::get(&buf) {
            if !kind.mime_type().starts_with("image/") {
                return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
            }
        } else {
            return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
        }

        let img = ImageReader::new(Cursor::new(&buf))
            .with_guessed_format()
            .map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)?
            .decode()
            .map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)?;

        let resized = img.resize(800, 800, FilterType::Lanczos3);

        let rgba = resized.to_rgba8();
        let encoder = Encoder::from_rgba(&rgba, resized.width(), resized.height());
        let webp_data = encoder.encode(80.0);

        // upload to minio using the s3 crate as tmp

        // add record on db with transaction

        // move it to final location

        // validate transaction

        Result::<StatusCode, StatusCode>::Ok(StatusCode::OK)
    }
}
