mod dumb;

use color_eyre::Report;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    info!("Hello from a comfy nest we've made for ourselves");
    let client = Client::new();

    const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
    const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";
    let url = "https://fasterthanli.me";

    fetch_thing(&client, url).await?;
    fetch_thing(&client, URL_1).await?;
    fetch_thing(&client, URL_2).await?;
    // this will turn non-200 HTTP status codes into rust errors,
    // so the first `?` propagates "we had a connection problem" and
    // the second `?` propagates "we had a chat with the server and they
    // were not pleased"
    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");

    Ok(())
}

async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
    let res = client.get(url).send().await?.error_for_status()?;

    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .json()
        // .with_env_filter(EnvFilter::from_default_env())
        // .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    Ok(())
}
