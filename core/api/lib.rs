use salvo::{catcher::Catcher, prelude::*, server::ServerHandle};
use tokio::signal;

mod article;
mod login;
mod router;
mod image;
mod system;
mod user;
use router::get_all_route;

/// 初始化路由
pub async fn init() {
    let router = get_all_route();

    let service = Service::new(router)
        .catcher(Catcher::default().hoop(hoop::catcher_all))
        .hoop(Logger::new())
        .hoop(hoop::cors_hoop());

    let config = config::get();

    let acceptor = TcpListener::new(&config.listen_addr).bind().await;

    println!(
        "访问地址: http://{}",
        &config.listen_addr.replace("0.0.0.0", "127.0.0.1")
    );

    let server = Server::new(acceptor);
    tokio::spawn(shutdown_signal(server.handle()));

    server.serve(service).await;
}

/// 优雅停机
async fn shutdown_signal(handle: ServerHandle) {
    // Wait Shutdown Signal
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("ctrl_c signal received"),
        _ = terminate => tracing::info!("terminate signal received"),
    }

    // Graceful Shutdown Server
    handle.stop_graceful(std::time::Duration::from_secs(60));
}
