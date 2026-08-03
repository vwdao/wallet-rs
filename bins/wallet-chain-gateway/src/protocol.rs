use url::Url;
use wallet_error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointProtocol {
    Http,
    WebSocket,
    Grpc,
    Tcp,
}

impl EndpointProtocol {
    pub fn supports_ws_tunnel(self) -> bool {
        matches!(self, Self::WebSocket | Self::Grpc | Self::Tcp)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::WebSocket => "ws",
            Self::Grpc => "grpc",
            Self::Tcp => "tcp",
        }
    }

    pub fn from_config(raw: &str) -> AppResult<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "http" | "https" => Ok(Self::Http),
            "ws" | "wss" => Ok(Self::WebSocket),
            "grpc" | "grpcs" => Ok(Self::Grpc),
            "tcp" => Ok(Self::Tcp),
            _ => Err(AppError::InvalidArgument(format!(
                "unsupported rpc protocol: {raw}"
            ))),
        }
    }

    pub fn from_config_opt(raw: &str) -> Option<Self> {
        if raw.is_empty() {
            return None;
        }
        Self::from_config(raw).ok()
    }
}

pub fn parse_endpoint_url(raw: &str) -> AppResult<(EndpointProtocol, Url)> {
    parse_endpoint_url_with(raw, None)
}

pub fn parse_endpoint_url_with(
    raw: &str,
    protocol: Option<EndpointProtocol>,
) -> AppResult<(EndpointProtocol, Url)> {
    let url = Url::parse(raw).map_err(|e| AppError::InvalidArgument(format!("invalid url: {e}")))?;
    let protocol = match protocol {
        Some(p) => p,
        None => protocol_from_scheme(url.scheme())?,
    };
    Ok((protocol, url))
}

fn protocol_from_scheme(scheme: &str) -> AppResult<EndpointProtocol> {
    match scheme {
        "http" | "https" => Ok(EndpointProtocol::Http),
        "ws" | "wss" => Ok(EndpointProtocol::WebSocket),
        "grpc" | "grpcs" => Ok(EndpointProtocol::Grpc),
        "tcp" => Ok(EndpointProtocol::Tcp),
        scheme => Err(AppError::InvalidArgument(format!(
            "unsupported url scheme: {scheme}"
        ))),
    }
}
