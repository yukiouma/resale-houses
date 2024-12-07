use area::{area::Area, usecase::AreaUsecase};
use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};

use crate::api::{
    area::{ListAreaReply, UpdateAreaPageRequest},
    errors::AppError,
};

pub fn area_router(usecase: AreaUsecase) -> Router {
    Router::new()
        .route("/", get(list_area))
        .route("/:id", put(update_area_page))
        .with_state(usecase)
}

async fn list_area(state: State<AreaUsecase>) -> anyhow::Result<Json<ListAreaReply>, AppError> {
    let areas = state.list_area().await?;
    Ok(Json(ListAreaReply { data: areas }))
}

async fn update_area_page(
    state: State<AreaUsecase>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateAreaPageRequest>,
) -> anyhow::Result<(), AppError> {
    let mut area = Area::default();
    area.id = Some(id);
    area.page = request.page;
    state.update_area_pages(&area).await?;
    Ok(())
}
