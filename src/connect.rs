//! Connection helper.
use tokio::net::TcpStream;

use tungstenite::{
    error::{Error, UrlError},
    handshake::client::Response,
    protocol::WebSocketConfig,
};

use crate::{domain, stream::MaybeTlsStream, IntoClientRequest, WebSocketStream};

#[cfg(any(feature = "native-tls", feature = "__rustls-tls"))]
use crate::Connector;

/// Connect to a given URL.
pub async fn connect_async<R>(
    request: R,
) -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    connect_async_with_config(request, None).await
}

/// The same as `connect_async()` but the one can specify a websocket configuration
/// Please refer to `connect_async()` for more details.
pub async fn connect_async_with_config<R>(
    request: R,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request = request.into_client_request()?;

    let domain = domain(&request)?;
    let port = request
        .uri()
        .port_u16()
        .or_else(|| match request.uri().scheme_str() {
            Some("wss") => Some(443),
            Some("ws") => Some(80),
            _ => None,
        })
        .ok_or(Error::Url(UrlError::UnsupportedUrlScheme))?;

    let addr = format!("{}:{}", domain, port);
    let try_socket = TcpStream::connect(addr).await;
    let socket = try_socket.map_err(Error::Io)?;

    connect_async_with_config_and_socket(request, config, socket).await
}

/// The same as `connect_async()` but the one can specify a websocket configuration
/// and pass a TcPStream.
/// Please refer to `connect_async()` for more details.
pub async fn connect_async_with_config_and_socket<R>(
    request: R,
    config: Option<WebSocketConfig>,
    socket: TcpStream,
) -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request = request.into_client_request()?;
    crate::tls::client_async_tls_with_config(request, socket, config, None).await
}

/// The same as `connect_async()` but the one can specify a websocket configuration,
/// and a TLS connector to use.
/// Please refer to `connect_async()` for more details.
#[cfg(any(feature = "native-tls", feature = "__rustls-tls"))]
pub async fn connect_async_tls_with_config<R>(
    request: R,
    config: Option<WebSocketConfig>,
    connector: Option<Connector>,
) -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request = request.into_client_request()?;
    
    let domain = domain(&request)?;
    let port = request
        .uri()
        .port_u16()
        .or_else(|| match request.uri().scheme_str() {
            Some("wss") => Some(443),
            Some("ws") => Some(80),
            _ => None,
        })
        .ok_or(Error::Url(UrlError::UnsupportedUrlScheme))?;

    let addr = format!("{}:{}", domain, port);
    let try_socket = TcpStream::connect(addr).await;
    let socket = try_socket.map_err(Error::Io)?;

    connect_async_tls_with_config_and_socket(request, config, connector, socket).await
}

/// The same as `connect_async()` but the one can specify a websocket configuration,
/// and a TLS connector to use and pass a TcpStream.
/// Please refer to `connect_async()` for more details.
#[cfg(any(feature = "native-tls", feature = "__rustls-tls"))]
pub async fn connect_async_tls_with_config_and_socket<R>(
    request: R,
    config: Option<WebSocketConfig>,
    connector: Option<Connector>,
    socket: TcpStream,
) -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request = request.into_client_request()?;
    crate::tls::client_async_tls_with_config(request, socket, config, connector).await
}
