use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::{
        middleware::{self, MiddlewareLayer},
        AppRoutes,
    },
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::openapi::ApiDocs;
#[allow(unused_imports)]
use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};

pub struct App;

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    #[cfg(debug_assertions)]
    async fn before_routes(ctx: &AppContext) -> Result<AxumRouter<AppContext>> {
        tracing::info!("Registering dev server routes!");

        Ok(crate::dev::DevServer::before_routes(AxumRouter::new(), ctx).await?)
    }

    async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Ok(router.merge(Scalar::with_url("/api", ApiDocs::openapi())))
    }

    #[cfg(debug_assertions)]
    fn middlewares(ctx: &AppContext) -> Vec<Box<dyn MiddlewareLayer>> {
        // Shortened reference to middlewares
        let middlewares = &ctx.config.server.middlewares;

        vec![
            // Limit Payload middleware with a default if none
            Box::new(middlewares.limit_payload.clone().unwrap_or_default()),
            // CORS middleware with a default if none
            Box::new(
                middlewares
                    .cors
                    .clone()
                    .unwrap_or_else(|| middleware::cors::Cors {
                        enable: false,
                        ..Default::default()
                    }),
            ),
            // Catch Panic middleware with a default if none
            Box::new(
                middlewares
                    .catch_panic
                    .clone()
                    .unwrap_or_else(|| middleware::catch_panic::CatchPanic { enable: true }),
            ),
            // Etag middleware with a default if none
            Box::new(
                middlewares
                    .etag
                    .clone()
                    .unwrap_or_else(|| middleware::etag::Etag { enable: true }),
            ),
            // Remote IP middleware with a default if none
            Box::new(middlewares.remote_ip.clone().unwrap_or_else(|| {
                middleware::remote_ip::RemoteIpMiddleware {
                    enable: false,
                    ..Default::default()
                }
            })),
            // Compression middleware with a default if none
            Box::new(
                middlewares
                    .compression
                    .clone()
                    .unwrap_or_else(|| middleware::compression::Compression { enable: false }),
            ),
            // Timeout Request middleware with a default if none
            Box::new(middlewares.timeout_request.clone().unwrap_or_else(|| {
                middleware::timeout::TimeOut {
                    enable: false,
                    ..Default::default()
                }
            })),
            // Static Assets middleware with a default if none
            Box::new(middlewares.static_assets.clone().unwrap_or_else(|| {
                middleware::static_assets::StaticAssets {
                    enable: false,
                    ..Default::default()
                }
            })),
            Box::new(crate::dev::DevServer),
            // Secure Headers middleware with a default if none
            Box::new(middlewares.secure_headers.clone().unwrap_or_else(|| {
                middleware::secure_headers::SecureHeader {
                    enable: false,
                    ..Default::default()
                }
            })),
            // Logger middleware with default logger configuration
            Box::new(middleware::logger::new(
                &middlewares
                    .logger
                    .clone()
                    .unwrap_or_else(|| middleware::logger::Config { enable: true }),
                &ctx.environment,
            )),
            // Request ID middleware with a default if none
            Box::new(
                middlewares
                    .request_id
                    .clone()
                    .unwrap_or_else(|| middleware::request_id::RequestId { enable: true }),
            ),
            // Fallback middleware with a default if none
            Box::new(middlewares.fallback.clone().unwrap_or_else(|| {
                middleware::fallback::Fallback {
                    enable: ctx.environment != Environment::Production,
                    ..Default::default()
                }
            })),
            // Powered by middleware with a default identifier
            Box::new(middleware::powered_by::new(
                ctx.config.server.ident.as_deref(),
            )),
        ]
    }

    #[cfg(not(debug_assertions))]
    fn middlewares(ctx: &AppContext) -> Vec<Box<dyn MiddlewareLayer>> {
        middleware::default_middleware_stack(ctx)
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::submission::routes())
            .add_route(controllers::course_page::routes())
            .add_route(controllers::assignment::routes())
            .add_route(controllers::enrollment::routes())
            .add_route(controllers::course::routes())
            .add_route(controllers::auth::routes())
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }

    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }

    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        Ok(())
    }
}
