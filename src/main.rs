mod helpers;
mod logic;
mod model;
mod routes;
mod settings;

use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use env_logger::WriteStyle;
use helpers::types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE};
use log::LevelFilter;
use routes::user::{api_get_user_by_id, api_get_user_by_id_astream};
use routes::webfinger::api_webfinger_query_resource;
use settings::SETTINGS;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let filter: LevelFilter = match SETTINGS.env {
    settings::AppEnv::Development => LevelFilter::Debug,
    settings::AppEnv::Testing => LevelFilter::Info,
    settings::AppEnv::Production => LevelFilter::Warn,
  };

  env_logger::Builder::new()
    .filter_level(filter)
    .write_style(WriteStyle::Always)
    .init();

  let pool = PgPoolOptions::new()
    .max_connections(SETTINGS.database.max_connections)
    .idle_timeout(Duration::from_secs(SETTINGS.database.idle_timeout.into()))
    .acquire_timeout(Duration::from_secs(SETTINGS.database.connection_timeout.into()))
    .connect(&SETTINGS.database.url)
    .await
    .unwrap();

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(web::Data::new(pool.clone()))
      .service(
        web::resource("/api/users/{handle}")
          .name("get_user_by_id")
          .guard(guard::Header("accept", ACTIVITY_JSON_CONTENT_TYPE.clone()))
          .to(api_get_user_by_id)
          .route(web::get().to(HttpResponse::Ok)),
      )
      .service(
        web::resource("/api/users/{handle}")
          .name("get_user_by_id")
          .guard(guard::Header("accept", ACTIVITY_LD_JSON_CONTENT_TYPE.clone()))
          .to(api_get_user_by_id_astream)
          .route(web::get().to(HttpResponse::Ok)),
      )
      .service(
        web::resource("/.well-known/webfinger")
          .name("get_user_by_id")
          .to(api_webfinger_query_resource)
          .route(web::get().to(HttpResponse::Ok)),
      )
  })
  .bind((SETTINGS.server.url.clone(), SETTINGS.server.port))?
  .run()
  .await
}
