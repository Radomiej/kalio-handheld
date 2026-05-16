mod app;
mod input;
mod layout;
mod network;
mod renderer;
mod scripting;
mod storage;

use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "kalio_runtime=debug".into()),
        )
        .init();

    info!("Kalio Runtime v{}", env!("CARGO_PKG_VERSION"));

    #[cfg(feature = "desktop-debug")]
    info!("desktop-debug: keyboard emulates gamepad (WASD/J/K/U/I)");

    app::run()
}
