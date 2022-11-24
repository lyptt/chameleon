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
mod routes;
mod settings;
mod work_queue;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use env_logger::WriteStyle;
use helpers::types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE, ApiError};
use log::LevelFilter;
use net::jwt_session::JwtSession;
use routes::comment::{
  api_create_comment, api_create_comment_like, api_delete_comment, api_delete_comment_like, api_get_comments,
};
use routes::follow::{api_create_follow, api_delete_follow};
use routes::job::api_job_query_status;
use routes::like::{api_create_like, api_delete_like};
use routes::oauth::{api_oauth_authorize, api_oauth_authorize_post, api_oauth_token};
use routes::post::{
  api_activitypub_get_user_public_feed, api_create_post, api_get_global_feed, api_get_post, api_get_user_own_feed,
  api_upload_post_image,
};
use routes::public::web_serve_static;
use routes::status::{api_get_server_status, ServerStatus, ServerComponentStatus};
use routes::user::{api_activitypub_get_user_by_id, api_activitypub_get_user_by_id_astream, api_get_profile};
use routes::webfinger::api_webfinger_query_resource;
use settings::SETTINGS;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{ComponentsBuilder, OpenApiBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
use work_queue::queue::Queue;
use model::{
  response::{JobResponse, ListResponse},
  comment_pub::CommentPub,
};

#[derive(OpenApi)]
#[openapi(components(schemas(ServerStatus, ServerComponentStatus, ApiError, JobResponse, ListResponse<CommentPub>)), paths(
  routes::comment::api_create_comment,
  routes::comment::api_create_comment_like,
  routes::comment::api_delete_comment,
  routes::comment::api_delete_comment_like,
  routes::comment::api_get_comments,
  routes::follow::api_create_follow,
  routes::follow::api_delete_follow,
  routes::job::api_job_query_status,
  routes::like::api_create_like,
  routes::like::api_delete_like,
  routes::oauth::api_oauth_authorize,
  routes::oauth::api_oauth_token,
  routes::post::api_activitypub_get_user_public_feed, 
  routes::post::api_create_post, 
  routes::post::api_get_global_feed, 
  routes::post::api_get_post, 
  routes::post::api_get_user_own_feed,
  routes::post::api_upload_post_image,
  routes::status::api_get_server_status,
  routes::user::api_activitypub_get_user_by_id,
  routes::user::api_activitypub_get_user_by_id_astream,
  routes::user::api_get_profile,
  routes::webfinger::api_webfinger_query_resource,
),
security(
  (),
  ("token_jwt" = [])
))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let filter: LevelFilter = match SETTINGS.env {
    settings::AppEnv::Development => LevelFilter::Debug,
    settings::AppEnv::Testing => LevelFilter::Info,
    settings::AppEnv::Production => LevelFilter::Warn,
  };

  AWSClient::create_s3_client().await;
  AWSClient::create_sqs_client().await;

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

    let openapi_raw_doc = ApiDoc::openapi();
    let builder: OpenApiBuilder = openapi_raw_doc.clone().into();
    let openapi = builder
      .components(Some(
        ComponentsBuilder::new()
          .schemas_from_iter(openapi_raw_doc.components.unwrap().schemas)
          .security_scheme(
            "token_jwt",
            SecurityScheme::Http(
              HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
            ),
          )
          .build(),
      ))
      .build();

    App::new()
      .wrap(Logger::default())
      .wrap(cors)
      .wrap(JwtSession::default())
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(Cdn::new()))
      .app_data(web::Data::new(Queue::new()))
      .service(
        web::resource("/api/activity/users/{handle}")
          .name("get_user_by_id")
          .route(
            web::get()
              .guard(guard::Header("accept", ACTIVITY_JSON_CONTENT_TYPE))
              .to(api_activitypub_get_user_by_id),
          ),
      )
      .service(
        web::resource("/api/activity/users/{handle}")
          .name("get_user_by_id")
          .route(
            web::get()
              .guard(guard::Header("accept", ACTIVITY_LD_JSON_CONTENT_TYPE))
              .to(api_activitypub_get_user_by_id_astream),
          ),
      )
      .service(
        web::resource("/api/activity/users/{handle}/feed")
          .name("get_user_public_feed")
          .route(
            web::get()
              .guard(guard::Header("accept", ACTIVITY_JSON_CONTENT_TYPE))
              .to(api_activitypub_get_user_public_feed),
          ),
      )
      .service(
        web::resource("/api/users/{user_handle}/follows")
          .name("user_follows")
          .route(web::post().to(api_create_follow))
          .route(web::delete().to(api_delete_follow)),
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
      .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(Url::new("api", "/api-doc/openapi.json"), openapi)]))
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
