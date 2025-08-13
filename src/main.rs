use quest_tracker_rust_mastery::{
    config::config_loader, infrastructure::postgres::postgres_connection,
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded successfully: {:?}", dotenvy_env);

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish PostgreSQL connection: {}", e);
            std::process::exit(1);
        }
    };

    info!("PostgreSQL connection established successfully");
}
