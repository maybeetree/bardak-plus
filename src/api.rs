use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

pub struct Api;

#[OpenApi]
impl Api {
    /// index
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText(
            format!(
                "Hello! I am {} version {}. \
                I am licensed under {}, \
                and my source code is at {}.",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_LICENSE"),
                env!("CARGO_PKG_REPOSITORY"),
            )
        )
    }
}

