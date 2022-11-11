mod activitypub;
mod aws;
mod cdn;
mod helpers;
mod logic;
mod model;
mod net;
mod routes;
mod settings;

use actix_web::dev::ServiceResponse;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use env_logger::WriteStyle;
use helpers::types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE};
use log::LevelFilter;
use net::jwt_session::JwtSession;
use routes::oauth::{api_oauth_authorize, api_oauth_authorize_post, api_oauth_token};
use routes::post::{api_activitypub_get_user_public_feed, api_get_user_own_feed, api_upload_post_image};
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

  AWSClient::create_s3_client().await;

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
      .wrap(JwtSession::default())
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(Cdn::new()))
      .service(
        web::resource("/api/users/{handle}")
          .name("get_user_by_id")
          .guard(guard::Header("accept", ACTIVITY_JSON_CONTENT_TYPE.clone()))
          .route(web::get().to(api_get_user_by_id)),
      )
      .service(
        web::resource("/api/users/{handle}")
          .name("get_user_by_id")
          .guard(guard::Header("accept", ACTIVITY_LD_JSON_CONTENT_TYPE.clone()))
          .route(web::get().to(api_get_user_by_id_astream)),
      )
      .service(
        web::resource("/api/users/{handle}/feed")
          .name("get_user_public_feed")
          .route(web::get().to(api_activitypub_get_user_public_feed)),
      )
      .service(
        web::resource("/api/posts/content")
          .name("upload_post_image")
          .route(web::post().to(api_upload_post_image)),
      )
      .service(
        web::resource("/.well-known/webfinger")
          .name("get_user_by_id")
          .route(web::get().to(api_webfinger_query_resource)),
      )
      .service(
        web::resource("/api/oauth/authorize")
          .name("oauth_authorize")
          .route(web::get().to(api_oauth_authorize))
          .route(web::post().to(api_oauth_authorize_post)),
      )
      .service(
        web::resource("/api/oauth/token")
          .name("oauth_token")
          .route(web::post().to(api_oauth_token)),
      )
      .service(
        web::resource("/api/feed")
          .name("oauth_token")
          .route(web::get().to(api_get_user_own_feed)),
      )
      .service(
        actix_files::Files::new("/", "./public/static")
          .prefer_utf8(true)
          .use_etag(true)
          // HACK: We use the file listing feature to return a blank 404 when the client requests
          //       a directory as actix_files doesn't provide a nice way for us to do this
          .show_files_listing()
          .files_listing_renderer(|_, req: &actix_web::HttpRequest| {
            Ok(ServiceResponse::new(req.clone(), HttpResponse::NotFound().finish()))
          }),
      )
  })
  .bind((SETTINGS.server.url.clone(), SETTINGS.server.port))?
  .run()
  .await
}
