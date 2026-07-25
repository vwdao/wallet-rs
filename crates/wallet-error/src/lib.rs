//! Unified application errors with HTTP / gRPC conversions.

use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;
use wallet_types::ChainIndex;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("chain not supported: {0}")]
    ChainNotSupported(ChainIndex),
    #[error("unavailable: {0}")]
    Unavailable(String),
    #[error("unimplemented: {0}")]
    Unimplemented(String),
    #[error("internal: {0}")]
    Internal(String),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl AppError {
    fn code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "not_found",
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::InvalidArgument(_) => "invalid_argument",
            Self::ChainNotSupported(_) => "chain_not_supported",
            Self::Unavailable(_) => "unavailable",
            Self::Unimplemented(_) => "unimplemented",
            Self::Internal(_) | Self::Other(_) => "internal",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::InvalidArgument(_) | Self::ChainNotSupported(_) => StatusCode::BAD_REQUEST,
            Self::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Unimplemented(_) => StatusCode::NOT_IMPLEMENTED,
            Self::Internal(_) | Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Scribe for AppError {
    fn render(self, res: &mut Response) {
        let status = self.status_code();
        let body = ErrorBody {
            code: self.code(),
            message: self.to_string(),
        };
        res.status_code(status);
        res.render(Json(body));
    }
}

impl From<AppError> for tonic::Status {
    fn from(value: AppError) -> Self {
        let code = match &value {
            AppError::NotFound(_) => tonic::Code::NotFound,
            AppError::Unauthorized => tonic::Code::Unauthenticated,
            AppError::Forbidden => tonic::Code::PermissionDenied,
            AppError::InvalidArgument(_) | AppError::ChainNotSupported(_) => {
                tonic::Code::InvalidArgument
            }
            AppError::Unavailable(_) => tonic::Code::Unavailable,
            AppError::Unimplemented(_) => tonic::Code::Unimplemented,
            AppError::Internal(_) | AppError::Other(_) => tonic::Code::Internal,
        };
        tonic::Status::new(code, value.to_string())
    }
}

impl From<tonic::Status> for AppError {
    fn from(value: tonic::Status) -> Self {
        match value.code() {
            tonic::Code::NotFound => AppError::NotFound(value.message().into()),
            tonic::Code::InvalidArgument => AppError::InvalidArgument(value.message().into()),
            tonic::Code::Unauthenticated => AppError::Unauthorized,
            tonic::Code::PermissionDenied => AppError::Forbidden,
            tonic::Code::Unimplemented => AppError::Unimplemented(value.message().into()),
            tonic::Code::Unavailable => AppError::Unavailable(value.message().into()),
            _ => AppError::internal(value.message()),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(serde::Serialize, salvo::oapi::ToSchema)]
pub struct ErrorResponse {
    /// Error code (e.g. "not_found", "unauthorized")
    pub code: &'static str,
    /// Human-readable error message
    pub message: String,
}

impl salvo::oapi::EndpointOutRegister for AppError {
    fn register(
        components: &mut salvo::oapi::Components,
        operation: &mut salvo::oapi::Operation,
    ) {
        use salvo::oapi::{Response, ToSchema};
        let schema = <ErrorResponse as ToSchema>::to_schema(components);
        operation.responses.insert(
            "400",
            Response::new("Bad request")
                .add_content("application/json", schema.clone()),
        );
        operation.responses.insert("401", Response::new("Unauthorized"));
        operation.responses.insert("403", Response::new("Forbidden"));
        operation.responses.insert(
            "404",
            Response::new("Not found")
                .add_content("application/json", schema.clone()),
        );
        operation.responses.insert(
            "500",
            Response::new("Internal error")
                .add_content("application/json", schema.clone()),
        );
        operation.responses.insert(
            "503",
            Response::new("Service unavailable")
                .add_content("application/json", schema),
        );
    }
}
