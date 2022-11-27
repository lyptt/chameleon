#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![deny(unused_imports)]

mod activitypub;
mod aws;
mod cdn;
mod helpers;
mod job;
mod logic;
mod model;
mod net;
mod rabbitmq;
mod routes;
mod settings;
mod work_queue;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use env_logger::WriteStyle;
use helpers::types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE};
use log::LevelFilter;
use net::jwt_session::JwtSession;
use rabbitmq::clients::RabbitMQClient;
use routes::comment::{
  api_create_comment, api_create_comment_like, api_delete_comment, api_delete_comment_like, api_get_comments,
};
use routes::follow::{api_create_follow, api_delete_follow};
use routes::job::api_job_query_status;
use routes::like::{api_create_like, api_delete_like};
use routes::oauth::{api_oauth_authorize, api_oauth_authorize_post, api_oauth_token};
use routes::post::{
  api_activitypub_get_user_public_feed, api_create_post, api_get_global_feed, api_get_post, api_get_user_own_feed,
  api_get_user_posts, api_upload_post_image,
};
use routes::public::web_serve_static;
use routes::status::api_get_server_status;
use routes::user::{
  api_activitypub_get_user_by_id, api_activitypub_get_user_by_id_astream, api_get_profile, api_get_user_profile,
  api_get_user_stats,
};
use routes::webfinger::api_webfinger_query_resource;
use settings::SETTINGS;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use work_queue::queue::Queue;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let filter: LevelFilter = match SETTINGS.env {
    settings::AppEnv::Development => LevelFilter::Debug,
    settings::AppEnv::Testing => LevelFilter::Info,
    settings::AppEnv::Production => LevelFilter::Warn,
  };

  AWSClient::create_s3_client().await;
  AWSClient::create_sqs_client().await;
  RabbitMQClient::create_rabbitmq_client().await;

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
    let cors = Cors::default()
      .allowed_origin_fn(|_, _| true)
      .allow_any_method()
      .allow_any_header()
      .supports_credentials()
      .max_age(3600);

    App::new()
      .wrap(Logger::default())
      .wrap(cors)
      .wrap(JwtSession::default())
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(Cdn::new()))
      .app_data(web::Data::new(Queue::new()))
      .service(
        web::resource("/api/users/{handle}")
          .name("get_user_by_id")
          .route(
            web::get()
              .guard(guard::Header("accept", ACTIVITY_JSON_CONTENT_TYPE))
              .to(api_activitypub_get_user_by_id),
          )
          .route(
            web::get()
              .guard(guard::Header("accept", ACTIVITY_LD_JSON_CONTENT_TYPE))
              .to(api_activitypub_get_user_by_id_astream),
          )
          .route(web::get().to(api_get_user_profile)),
      )
      .service(
        web::resource("/api/users/{handle}/feed")
          .name("get_user_public_feed")
          .route(
            web::get()
              .guard(guard::Header("accept", ACTIVITY_JSON_CONTENT_TYPE))
              .to(api_activitypub_get_user_public_feed),
          )
          .route(web::get().to(api_get_user_posts)),
      )
      .service(
        web::resource("/api/users/{user_handle}/follows")
          .name("user_follows")
          .route(web::post().to(api_create_follow))
          .route(web::delete().to(api_delete_follow)),
      )
      .service(
        web::resource("/api/users/{handle}/stats")
          .name("user_stats")
          .route(web::get().to(api_get_user_stats)),
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
          .name("feed")
          .route(web::get().to(api_get_user_own_feed))
          .route(web::post().to(api_create_post)),
      )
      .service(
        web::resource("/api/feed/federated")
          .name("federated_feed")
          .route(web::get().to(api_get_global_feed)),
      )
      .service(
        web::resource("/api/feed/{post_id}")
          .name("upload_post_image")
          .route(web::get().to(api_get_post))
          .route(web::post().to(api_upload_post_image)),
      )
      .service(
        web::resource("/api/feed/{post_id}/likes")
          .name("post_likes")
          .route(web::post().to(api_create_like))
          .route(web::delete().to(api_delete_like)),
      )
      .service(
        web::resource("/api/feed/{post_id}/comments")
          .name("post_comments")
          .route(web::get().to(api_get_comments))
          .route(web::post().to(api_create_comment)),
      )
      .service(
        web::resource("/api/feed/{post_id}/comments/{comment_id}/likes")
          .name("post_comment_likes")
          .route(web::post().to(api_create_comment_like))
          .route(web::delete().to(api_delete_comment_like)),
      )
      .service(
        web::resource("/api/feed/{post_id}/comments/{comment_id}")
          .name("post_comment")
          .route(web::delete().to(api_delete_comment)),
      )
      .service(
        web::resource("/api/profile")
          .name("profile")
          .route(web::get().to(api_get_profile)),
      )
      .service(
        web::resource("/api/job/{job_id}")
          .name("jobs")
          .route(web::get().to(api_job_query_status)),
      )
      .service(
        web::resource("/.well-known/webfinger")
          .name("get_user_by_id")
          .route(web::get().to(api_webfinger_query_resource)),
      )
      .service(
        web::resource("/.well-known/status")
          .name("status")
          .route(web::get().to(api_get_server_status)),
      )
      .service(
        web::resource("/{path:.*}")
          .name("static_files")
          .route(web::get().to(web_serve_static)),
      )
  })
  .bind((SETTINGS.server.url.clone(), SETTINGS.server.port))?
  .run()
  .await
}
