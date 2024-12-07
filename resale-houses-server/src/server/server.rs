use super::router::area::area_router;
use area::usecase::AreaUsecase;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(area: AreaUsecase) -> Server {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        let area_router = area_router(area);
        let router = Router::new()
            .nest("/area", area_router)
            .layer(TraceLayer::new_for_http());
        Server { router }
    }
    pub async fn run(self, port: usize) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("Resale houses server listening port: {}", port);
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
