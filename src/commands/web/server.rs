use super::assets::WebAssets;
use crate::config::Manager;
use crate::diff::{Diff, DiffProvider};
use crate::git::{GitProvider, core::SourceKind};
use axum::Json;
use axum::extract::State;
use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode, Uri, header},
    response::IntoResponse,
    routing::get,
};
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Notify;

#[derive(Clone)]
pub struct WebServerState {
    pub git: Arc<dyn GitProvider + Send + Sync>,
    pub base_branch: String,
    pub target_branch: String,
}

pub async fn start_server(
    port: u16,
    config: &mut dyn Manager,
    git: Arc<dyn GitProvider + Send + Sync>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = config.load().map_err(|e| {
        io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to load config: {}", e),
        )
    })?;
    let base = data.base_branch().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "base_branchが設定されていません。mirudi init を先に実行してください",
        )
    })?;

    let target = data.current_branch().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "current_branchが設定されていません。mirudi scope を先に実行してください",
        )
    })?;

    let state = WebServerState {
        git,
        base_branch: base,
        target_branch: target,
    };

    let state = Arc::new(state);

    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_notify_for_exit = shutdown_notify.clone();

    let api_routes = Router::new()
        .route("/changes", get(get_changed_files))
        .route("/diffs", get(get_diffs));

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
        app.with_state(state),
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

async fn get_changed_files(State(state): State<Arc<WebServerState>>) -> impl IntoResponse {
    let base = &state.base_branch;
    let target = &state.target_branch;
    let git = state.git.clone();

    match git.list_changed_files(base, target) {
        Ok(files) => Json(files).into_response(),
        Err(e) => {
            eprintln!("エラー発生: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "変更ファイル取得失敗").into_response()
        }
    }
}

async fn get_diffs(State(state): State<Arc<WebServerState>>) -> impl IntoResponse {
    let base = &state.base_branch;
    let target = &state.target_branch;
    let git = state.git.clone();

    match git.list_changed_files(base, target) {
        Ok(files) => {
            let mut diffs = HashMap::new();

            for file_path in files {
                let old_lines =
                    git.extract_lines(base, &file_path, 1, usize::MAX, Some(SourceKind::Commit));
                let new_lines =
                    git.extract_lines(target, &file_path, 1, usize::MAX, Some(SourceKind::Commit));

                match (old_lines, new_lines) {
                    (Ok(old), Ok(new)) => {
                        let diff = Diff::new(old, new).lines_structured();
                        diffs.insert(file_path, diff);
                    }
                    _ => {
                        eprintln!("ファイルのdiff取得失敗: {}", file_path);
                    }
                }
            }

            Json(diffs).into_response()
        }
        Err(e) => {
            eprintln!("エラー発生: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "diff一覧取得失敗").into_response()
        }
    }
}
