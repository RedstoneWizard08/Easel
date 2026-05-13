#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::enrollments::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::ToResponse)]
pub struct Params {
    pub user_id: i32,
    pub course_id: i32,
    pub until: DateTime,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.user_id = Set(self.user_id);
        item.course_id = Set(self.course_id);
        item.until = Set(self.until);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
#[utoipa::path(get, path = "/api/enrollments", responses((status = 200, body = Vec<Model>)))]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
#[utoipa::path(post, path = "/api/enrollments", responses((status = 200, body = Model)))]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
#[utoipa::path(method(put, patch), path = "/api/enrollments/{id}", responses((status = 200, body = Model)))]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
#[utoipa::path(delete, path = "/api/enrollments/{id}")]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
#[utoipa::path(get, path = "/api/enrollments/{id}", responses((status = 200, body = Model)))]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/enrollments/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
