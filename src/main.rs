#![allow(dead_code)]

use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use reqwest::Client;
use std::future::Future;
use std::sync::Arc;

pub const URL_1: &str = "https://jhaveri.net";
pub const URL_2: &str = "https://fasterthanli.me";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Hi from the program");
    let client = Client::new();
    let client = Arc::new(client);


    let fut1 = fetch(client.clone(), URL_1);
    let fut2 = fetch(client.clone(), URL_2);

    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);

    handle2.await.unwrap()?;
    handle1.await.unwrap()?;

    Ok(())
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

fn fetch(client: Arc<Client>, url: &'static str) -> impl Future<Output=Result<(), Report>> + 'static {
    async move {
        let res = client.get(url).send().await?.error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response");

        Ok(())
    }
}
