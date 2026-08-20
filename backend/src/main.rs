use axum::{
    routing::{get, post, patch, delete},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod handlers;
mod middleware;
mod services;
mod websocket;

use config::Config;
use db::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "liecord_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting LieCord Backend on {}:{}", config.host, config.port);

    // Initialize database connections
    let db = Database::new(&config).await?;
    let db = Arc::new(db);

    // Build application state
    let state = Arc::new(AppState {
        db: db.clone(),
        config: config.clone(),
    });

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        
        // Auth routes
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/me", get(handlers::auth::get_current_user))
        
        // User routes
        .route("/api/users/:id", get(handlers::users::get_user))
        .route("/api/users/profile", patch(handlers::users::update_profile))
        .route("/api/users/avatar", post(handlers::users::upload_avatar))
        .route("/api/users/banner", post(handlers::users::upload_banner))
        .route("/api/users/status", patch(handlers::users::update_status))
        .route("/api/users/friend-request/:id", post(handlers::users::send_friend_request))
        .route("/api/users/friend-request/:id/accept", post(handlers::users::accept_friend_request))
        .route("/api/users/friend/:id", delete(handlers::users::remove_friend))
        
        // Server routes
        .route("/api/servers", post(handlers::servers::create_server))
        .route("/api/servers", get(handlers::servers::get_user_servers))
        .route("/api/servers/:id", get(handlers::servers::get_server))
        .route("/api/servers/:id", patch(handlers::servers::update_server))
        .route("/api/servers/:id", delete(handlers::servers::delete_server))
        .route("/api/servers/:id/icon", post(handlers::servers::upload_icon))
        .route("/api/servers/:id/invite", post(handlers::servers::create_invite))
        .route("/api/servers/join/:code", post(handlers::servers::join_server))
        .route("/api/servers/:id/leave", delete(handlers::servers::leave_server))
        
        // Channel routes
        .route("/api/channels", post(handlers::channels::create_channel))
        .route("/api/channels/:id", get(handlers::channels::get_channel))
        .route("/api/channels/:id", patch(handlers::channels::update_channel))
        .route("/api/channels/:id", delete(handlers::channels::delete_channel))
        .route("/api/channels/:id/messages", get(handlers::channels::get_messages))
        
        // Message routes
        .route("/api/messages", post(handlers::messages::send_message))
        .route("/api/messages/:id", patch(handlers::messages::edit_message))
        .route("/api/messages/:id", delete(handlers::messages::delete_message))
        .route("/api/messages/:id/react", post(handlers::messages::add_reaction))
        .route("/api/messages/:id/react", delete(handlers::messages::remove_reaction))
        .route("/api/messages/search", post(handlers::messages::search_messages))
        
        // DM routes
        .route("/api/dm", post(handlers::dms::create_or_get_dm))
        .route("/api/dm", get(handlers::dms::get_user_dms))
        .route("/api/dm/:id/messages", get(handlers::dms::get_dm_messages))
        
        // Nitro routes
        .route("/api/nitro/subscribe", post(handlers::nitro::subscribe))
        .route("/api/nitro/cancel", post(handlers::nitro::cancel))
        .route("/api/nitro/renew", post(handlers::nitro::renew))
        .route("/api/nitro/info", get(handlers::nitro::get_info))
        .route("/api/nitro/boost/:server_id", post(handlers::nitro::boost_server))
        
        // WebSocket route
        .route("/ws", get(websocket::ws_handler))
        
        // Serve uploaded files
        .nest_service("/uploads", tower_http::services::ServeDir::new(&config.upload_dir))
        
        // Middleware
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    tracing::info!("🚀 LieCord Backend listening on {}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Config,
}
