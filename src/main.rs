#![allow(dead_code)]

use color_eyre::Report;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, rustls::ClientConfig, webpki::DNSNameRef};
use tracing::info;
use tracing_subscriber::EnvFilter;

use webpki_roots;

mod tj;

pub const URL_1: &str = "https://jhaveri.net";
pub const URL_2: &str = "https://jhaveri.net/chess/about.html";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Hi from the program");

    let fut2 = fetch(URL_2);
    let fut1 = fetch(URL_1);

    let res = tj::try_join(fut1, fut2,).await?;
    info!(?res, "all done");

    Ok(())
}

const HTTP_REQ: &[u8] = b"GET / HTTP/1.1\r
HOST: jhaveri.net\r
User-Agent: cool-bear\r
Connection: close\r
\r
";

async fn fetch(name: &str) -> Result<&str, Report> {
    let addr = SocketAddr::from(([138, 68, 195, 194], 443));
    let socket = TcpStream::connect(addr).await?;

    let connector: TlsConnector = {
        let mut config = ClientConfig::new();
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        Arc::new(config).into()
    };

    let dnsname = DNSNameRef::try_from_ascii_str("jhaveri.net")?;
    let mut socket = connector.connect(dnsname, socket).await?;

    socket.write_all(HTTP_REQ).await?;

    let mut response = String::with_capacity(256);
    socket.read_to_string(&mut response).await?;

    let status = response.lines().next().unwrap_or_default();
    info!(%status, %name, "Got Response!");

    Ok(name)
}

fn type_name_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
