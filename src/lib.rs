use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_service;

/// A simple Spin HTTP component.
#[http_service]
async fn handle_spin4_internal_broken(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!(
        "Handling request to {:?}",
        req.headers().get("spin-full-url")
    );

    spin_sdk::http::get("http://one.spin.internal").await?;
    spin_sdk::http::get("http://two.spin.internal").await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello World!".to_string()))
}
