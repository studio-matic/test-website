use crate::{ApiResult, auth::validate};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::MySqlPool;
use thiserror::Error;
use time::OffsetDateTime;

#[derive(utoipa::OpenApi)]
#[openapi(paths(supporters))]
struct ApiDoc;
pub fn openapi() -> utoipa::openapi::OpenApi {
    use utoipa::OpenApi;
    ApiDoc::openapi()
}

#[derive(Error, Debug)]
pub enum SupporterError {
    #[error("Could not format")]
    FormatError(#[from] time::error::Format),
    #[error("Could not query database")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for SupporterError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::FormatError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let msg = self.to_string();

        (status, Json(msg)).into_response()
    }
}

#[derive(Serialize, utoipa::ToSchema)]
struct SupporterResponse {
    id: u64,
    name: String,
    supported_at: String,
    income_eur: f64,
    co_op: String,
}

#[utoipa::path(
    get,
    path = "/supporters",
    responses(
        (
            status = StatusCode::OK,
            body = Vec<SupporterResponse>,
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Missing session_token",
        ),
        (
            status = StatusCode::UNAUTHORIZED,
            description = "Not logged in",
        ),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
)]
pub async fn supporters(
    state_pool: State<MySqlPool>,
    headers: HeaderMap,
) -> ApiResult<impl IntoResponse> {
    let _ = validate(state_pool.clone(), headers).await?;
    let supporters: Vec<(u64, String, OffsetDateTime, f64, String)> = sqlx::query_as(
        "SELECT supporters.id, supporters.name, donations.donated_at, donations.income_eur, donations.co_op
            FROM supporters
            JOIN donations ON donations.id = supporters.donation_id",
    )
    .fetch_all(&state_pool.0)
    .await
    .map_err(SupporterError::DatabaseError)?;

    let supporters = supporters
        .into_iter()
        .map(|(a, b, c, d, e)| {
            Ok(SupporterResponse {
                id: a,
                name: b,
                supported_at: c
                    .to_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .map_err(SupporterError::FormatError)?,
                income_eur: d,
                co_op: e,
            })
        })
        .collect::<ApiResult<Vec<_>>>()?;

    Ok((StatusCode::OK, Json(supporters)))
}
