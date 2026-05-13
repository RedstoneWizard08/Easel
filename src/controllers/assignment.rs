#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::assignments::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::ToResponse)]
pub struct Params {
    pub name: String,
    pub description: Option<String>,
    pub due: Option<DateTime>,
    pub lock_at: Option<DateTime>,
    pub lock_until: Option<DateTime>,
    pub max_score: Option<i32>,
    pub is_quiz: Option<bool>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.description = Set(self.description.clone());
        item.due = Set(self.due);
        item.lock_at = Set(self.lock_at);
        item.lock_until = Set(self.lock_until);
        item.max_score = Set(self.max_score);
        item.is_quiz = Set(self.is_quiz);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[utoipa::path(get, path = "/api/assignments", responses((status = 200, body = Vec<Model>)))]
#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[utoipa::path(post, path = "/api/assignments", responses((status = 200, body = Model)))]
#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[utoipa::path(method(put, patch), path = "/api/assignments/{id}", responses((status = 200, body = Model)))]
#[debug_handler]
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

#[utoipa::path(delete, path = "/api/assignments/{id}")]
#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[utoipa::path(get, path = "/api/assignments/{id}", responses((status = 200, body = Model)))]
#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/assignments/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
