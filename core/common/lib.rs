use salvo::http::{ParseError, StatusCode, StatusError};
use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = Result<Json<T>, AppError>;
pub type EmptyResult = Result<Json<Empty>, AppError>;

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(Json(data))
}
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Empty {}
pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(Empty {}))
}
#[derive(Error, Debug)]
pub enum AppError {
    #[error("public: `{0}`")]
    Public(String),
    #[error("internal: `{0}`")]
    Internal(String),
    #[error("salvo internal error: `{0}`")]
    Salvo(#[from] salvo::Error),
    #[error("http status error: `{0}`")]
    HttpStatus(#[from] StatusError),
    #[error("http parse error:`{0}`")]
    HttpParse(#[from] ParseError),
    #[error("seaorm db error:`{0}`")]
    Seaorm(#[from] sea_orm::DbErr),
    #[error("jsonwebtoken error:`{0}`")]
    JwtErr(#[from] jsonwebtoken::errors::Error),
    #[error("validation error:`{0}`")]
    Validation(#[from] validator::ValidationErrors),
    #[error("anyhow error:`{0}`")]
    Anyhow(#[from] anyhow::Error),
}
impl AppError {
    pub fn public<S: Into<String>>(msg: S) -> Self {
        Self::Public(msg.into())
    }

    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    error_type: &'static str,
    message: String,
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let mut response = ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error_type: "internal_server_error",
            message: "unknown_error".to_string(),
        };
        match &self {
            AppError::Public(msg) => {
                response.status = StatusCode::BAD_REQUEST.as_u16();
                response.error_type = "client_error";
                response.message = msg.to_string();
            }
            AppError::Internal(msg) => {
                response.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                response.error_type = "internal_server_error";
                response.message = msg.to_string();
            }
            AppError::Salvo(e) => {
                response.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                response.error_type = "salvo_error";
                response.message = e.to_string();
            }
            AppError::HttpStatus(e) => {
                response.status = e.code.as_u16();
                response.error_type = "http_status_error";
                response.message = e.to_string();
            }
            AppError::HttpParse(e) => {
                response.status = StatusCode::BAD_REQUEST.as_u16();
                response.error_type = "parse_error";
                response.message = e.to_string();
            }
            AppError::Seaorm(e) => {
                response.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                response.error_type = "database_error";
                response.message = e.to_string();
            }
            AppError::JwtErr(e) => {
                response.status = StatusCode::UNAUTHORIZED.as_u16();
                response.error_type = "authentication_error";
                response.message = e.to_string();
            }
            AppError::Validation(e) => {
                response.status = StatusCode::UNPROCESSABLE_ENTITY.as_u16();
                response.error_type = "validation_error";
                response.message = e.to_string();
            }
            _ => {
                tracing::error!(error = ?self, "Unexpected error occurred");
            }
        };

        // 设置响应
        res.status_code(StatusCode::from_u16(response.status).unwrap());
        res.render(Json(response));
    }
}
