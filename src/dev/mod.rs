pub mod handler;
pub mod state;
pub mod ws;

use std::path::PathBuf;

use crate::dev::state::ProxyState;
use axum::{routing::get, Extension, Router as AxumRouter};
use loco_rs::{app::AppContext, controller::middleware::MiddlewareLayer, Result};
use serde_json::Value;
use tokio::process::Command;

pub struct DevServer;

impl DevServer {
    pub async fn before_routes(
        router: AxumRouter<AppContext>,
        _ctx: &AppContext,
    ) -> Result<AxumRouter<AppContext>> {
        tokio::task::spawn(async move {
            let dir = PathBuf::from(env!("FRONTEND_DIR"));

            Command::new("bun")
                .arg("run")
                .arg("dev")
                .current_dir(dir)
                .kill_on_drop(true)
                .spawn()
                .unwrap()
                .wait()
                .await
        });

        // let state = ProxyState::new("http://localhost:5151".into());

        // Ok(router
        //     .route("/rsbuild-hmr", get(ws::websocket_handler))
        //     .fallback(handler::fallback_handler)
        //     .layer(Extension(state)))

        Ok(router)
    }

    pub fn register<T: Clone + Send + Sync + 'static>(
        router: AxumRouter<T>,
    ) -> Result<AxumRouter<T>> {
        let state = ProxyState::new("http://localhost:5151".into());

        Ok(router
            .route("/rsbuild-hmr", get(ws::websocket_handler))
            .fallback(handler::fallback_handler)
            .layer(Extension(state)))
    }

    pub async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Self::register(router)
    }
}

impl MiddlewareLayer for DevServer {
    fn name(&self) -> &'static str {
        "dev_server"
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn config(&self) -> serde_json::Result<Value> {
        Ok(Value::Null)
    }

    fn apply(&self, app: AxumRouter<AppContext>) -> Result<AxumRouter<AppContext>> {
        Self::register(app)
    }
}
