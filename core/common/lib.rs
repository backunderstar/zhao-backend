use salvo::http::{ParseError, StatusCode, StatusError};
use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = Result<Json<AppResponse<T>>, AppError>;
pub type EmptyResult = Result<Json<AppResponse<Empty>>, AppError>;

#[derive(Serialize, Clone, Debug)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(Json(AppResponse {
        code: 200,
        message: "success".to_string(),
        data,
    }))
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Empty {}

pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(AppResponse {
        code: 200,
        message: "success".to_string(),
        data: Empty {},
    }))
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
    
    // 获取错误对应的状态码
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Public(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Salvo(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::HttpStatus(e) => e.code,
            AppError::HttpParse(_) => StatusCode::BAD_REQUEST,
            AppError::Seaorm(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtErr(_) => StatusCode::UNAUTHORIZED,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    // 获取错误消息
    fn message(&self) -> String {
        match self {
            AppError::Public(msg) => msg.clone(),
            AppError::Internal(msg) => msg.clone(),
            AppError::Salvo(e) => e.to_string(),
            AppError::HttpStatus(e) => e.to_string(),
            AppError::HttpParse(e) => e.to_string(),
            AppError::Seaorm(e) => e.to_string(),
            AppError::JwtErr(e) => e.to_string(),
            AppError::Validation(e) => e.to_string(),
            AppError::Anyhow(e) => e.to_string(),
        }
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status_code = self.status_code();
        let message = self.message();
        
        // 记录未预期的错误
        if matches!(self, AppError::Anyhow(_)) {
            tracing::error!(error = ?self, "Unexpected error occurred");
        }

        // 设置响应状态码
        res.status_code(status_code);
        
        // 使用统一的响应格式
        res.render(Json(AppResponse {
            code: status_code.as_u16(),
            message,
            data: Empty {},
        }));
    }
}
