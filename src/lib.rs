mod error;
#[macro_use]
mod macros;
mod models;
mod repository;
mod routes;
mod schema;
mod service;
mod state;
mod templates;
mod utils;

use axum::{http::{header::CONTENT_TYPE, Method}, Extension, Router};
use dotenv::dotenv;
use repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait};
use routes::create_api_routes;
use sqlx::{Pool, Sqlite};
use templates::create_ui_routes;
use std::{env, io, sync::{LazyLock, RwLock}, time::Duration};
use tera::Tera;
use tera_hot_reload::{watch, LiveReloadLayer};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::{Any, CorsLayer}, services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use schema::connect_db;
use service::shortcut::ShortcutService;
use state::AppState;

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    RwLock::new(tera::Tera::new("ui/templates/**/*").expect("Failed to create Tera instance"))
});

pub async fn app() -> Result<Router, ()> {
    let livereload: LiveReloadLayer = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let connection: Pool<Sqlite> = connect_db().await;
    info!("connected to database");
    
    let shortcut_repository: ShortcutRepository = ShortcutRepository::new(connection);

    let shortcut_service = ShortcutService::new(shortcut_repository);

    let state: AppState = AppState::new(shortcut_service);
    info!("done intializing appstate");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app: Router = Router::new()
        .merge(create_ui_routes())
        .nest_service("/assets", ServeDir::new("ui/assets"))
        .nest("/api", create_api_routes())
        .layer(livereload)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(Extension(state));

    let _debouncer = watch(
        move || {
            info!("Reloading...");
            let _ = TERA.write().unwrap().full_reload();
            reloader.reload();
        },
        Duration::from_millis(10), // if you have tailwindcss and your machine is slow, you can increase this value
        vec!["./ui/templates"] // this is now listening for changes in the templates folder add any other folders you want to watch this can be your folder that holds your JS files or CSS or whatever you are serving in your app
    );
    info!("Hot reload set-up complete.");
    
    Ok(app)
}

pub async fn run() {
    dotenv().ok();
    let base_store_path = std::env::var("BASE_STORE_PATH").unwrap_or(String::from(""));
    eprintln!("{}", base_store_path);
    let file_appender = rolling::never(base_store_path.clone() + "/logs", "full.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    Box::leak(Box::new(guard)); 

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "{}=debug,tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    });

    let file_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(file_writer);

    tracing_subscriber::registry()
        .with(
            // stdout layer, to view everything in the console
            fmt::layer()
                .compact()
                .with_writer(io::stderr)
                .with_filter(env_filter)
        )
        .with(
            file_layer
                .with_ansi(false)
                .with_filter(filter::LevelFilter::DEBUG),
        )
        .init();

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));

    let addr = format!("0.0.0.0:{}", port);
    info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    let app = app().await.unwrap();

    axum::serve(listener, app).await.unwrap();
}