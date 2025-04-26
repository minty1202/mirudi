use super::assets::WebAssets;
use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode, Uri, header},
    response::IntoResponse,
    routing::get,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Notify;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_notify_for_exit = shutdown_notify.clone();

    let api_routes = Router::new().route("/sample", get(|| async { "Hello, World!" }));

    let app = Router::new()
        .nest("/api", api_routes)
        .route("/", get(static_handler))
        .route("/{*path}", get(static_handler))
        .route(
            "/exit",
            get(move || {
                let notify = shutdown_notify_for_exit.clone();
                async move {
                    println!("サーバーを終了します。");
                    notify.notify_waiters();
                    "サーバーを終了します"
                }
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("サーバーを起動しました: http://{}", addr);

    webbrowser::open(format!("http://{}", addr).as_str())?;

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .with_graceful_shutdown(async move {
        shutdown_notify.notified().await;
    })
    .await?;

    Ok(())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match WebAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data.to_vec()))
                .unwrap()
        }
        None => match WebAssets::get("index.html") {
            Some(content) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html")
                .body(Body::from(content.data.to_vec()))
                .unwrap(),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))
                .unwrap(),
        },
    }
}
