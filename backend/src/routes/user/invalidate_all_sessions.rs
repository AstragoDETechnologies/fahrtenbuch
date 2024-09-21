use crate::data_models::session::SessionInput;
use crate::DB;
use crate::{routes::ApiResponse, util::user::invalidate_all_sessions::invalidate_all_sessions};
use axum::{http::StatusCode, Json};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

pub async fn post_invalidate_all_sessions(
    Json(payload): Json<SessionInput>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
    let pool = match DB.get() {
        Some(pool) => pool,
        None => {
            error!("Could not get database connection.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(String::from("Could not get database connection.")),
                    data: None,
                }),
            );
        }
    };

    match invalidate_all_sessions(&payload, pool).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    is_error: false,
                    error_msg: None,
                    data: Some(true),
                }),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    is_error: true,
                    error_msg: Some(format!(
                        "An Error occurred during the invalidation: {:?}",
                        e
                    )),
                    data: None,
                }),
            );
        }
    };
}
