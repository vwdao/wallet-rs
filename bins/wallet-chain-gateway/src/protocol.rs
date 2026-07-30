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
}

pub fn parse_endpoint_url(raw: &str) -> AppResult<(EndpointProtocol, Url)> {
    let url = Url::parse(raw).map_err(|e| AppError::InvalidArgument(format!("invalid url: {e}")))?;
    let protocol = match url.scheme() {
        "http" | "https" => EndpointProtocol::Http,
        "ws" | "wss" => EndpointProtocol::WebSocket,
        "grpc" | "grpcs" => EndpointProtocol::Grpc,
        "tcp" => EndpointProtocol::Tcp,
        scheme => {
            return Err(AppError::InvalidArgument(format!(
                "unsupported url scheme: {scheme}"
            )));
        }
    };
    Ok((protocol, url))
}
