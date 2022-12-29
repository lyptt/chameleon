#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![deny(unused_imports)]

mod activitypub;
mod aws;
mod cdn;
mod db;
mod federation;
mod helpers;
mod job;
mod logic;
mod model;
mod net;
mod rabbitmq;
mod routes;
mod settings;
mod work_queue;

use std::time::Duration;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use db::repository::Repository;
use deadpool::Runtime;
use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use env_logger::WriteStyle;
use helpers::types::{ACTIVITYPUB_ACCEPT_GUARD, HTML_GUARD};
use log::LevelFilter;
use net::jwt_session::JwtSession;
use rabbitmq::clients::RabbitMQClient;
use routes::activitypub::{
  api_activitypub_federate_shared_inbox, api_activitypub_federate_user_inbox, api_activitypub_get_comment,
  api_activitypub_get_comments, api_activitypub_get_federated_user_liked_posts,
  api_activitypub_get_federated_user_posts, api_activitypub_get_post, api_activitypub_get_user_followers,
  api_activitypub_get_user_following, api_activitypub_get_user_profile,
};
use routes::apps::api_create_app;
use routes::comment::{
  api_create_comment, api_create_comment_like, api_delete_comment, api_delete_comment_like, api_get_comment,
  api_get_comments,
};
use routes::follow::{api_create_follow, api_delete_follow};
use routes::host_meta::api_get_host_meta;
use routes::job::api_job_query_status;
use routes::like::{api_create_like, api_delete_like};
use routes::nodeinfo::{api_get_nodeinfo, api_get_nodeinfo_2_1};
use routes::oauth::{api_oauth_authorize, api_oauth_authorize_post, api_oauth_token};
use routes::orbit::{
  api_create_orbit, api_create_orbit_moderator, api_delete_orbit, api_delete_orbit_moderator, api_get_orbit,
  api_get_orbit_moderators, api_get_orbit_named, api_get_orbits, api_get_user_orbits, api_join_orbit, api_leave_orbit,
  api_update_orbit, api_update_orbit_assets, api_update_orbit_moderator,
};
use routes::post::{
  api_boost_post, api_create_post, api_get_global_feed, api_get_orbit_feed, api_get_post, api_get_user_liked_posts,
  api_get_user_own_feed, api_get_user_post, api_get_user_posts, api_unboost_post, api_upload_post_image,
};
use routes::public::web_serve_static;
use routes::redirect::{
  api_redirect_to_federated_user_liked_posts, api_redirect_to_federated_user_posts, api_redirect_to_post,
  api_redirect_to_post_comment, api_redirect_to_post_comments, api_redirect_to_user, api_redirect_to_user_followers,
  api_redirect_to_user_following,
};
use routes::status::api_get_server_status;
use routes::user::{
  api_get_profile, api_get_user_followers, api_get_user_following, api_get_user_profile, api_get_user_stats,
};
use routes::webfinger::api_webfinger_query_resource;
use settings::SETTINGS;
use tokio_postgres::NoTls;
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

  let pool = {
    let mut cfg = deadpool_postgres::Config::new();
    cfg.host = Some(SETTINGS.database.host.to_owned());
    cfg.port = Some(SETTINGS.database.port);
    cfg.dbname = Some(SETTINGS.database.database.to_owned());
    cfg.user = Some(SETTINGS.database.username.to_owned());
    cfg.password = Some(SETTINGS.database.password.to_owned());
    cfg.manager = Some(ManagerConfig {
      recycling_method: RecyclingMethod::Verified,
    });
    cfg.keepalives_idle = Some(Duration::from_secs((SETTINGS.database.idle_timeout * 60).into()));
    cfg.connect_timeout = Some(Duration::from_secs(SETTINGS.database.connection_timeout.into()));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    pool.resize(SETTINGS.database.max_connections);
    pool
  };

  let app_pool = Repository::new_app_pool(&pool);
  let comment_pool = Repository::new_comment_pool(&pool);
  let event_pool = Repository::new_event_pool(&pool);
  let follow_pool = Repository::new_follow_pool(&pool);
  let job_pool = Repository::new_job_pool(&pool);
  let like_pool = Repository::new_like_pool(&pool);
  let post_pool = Repository::new_post_pool(&pool);
  let post_attachment_pool = Repository::new_post_attachment_pool(&pool);
  let session_pool = Repository::new_session_pool(&pool);
  let user_pool = Repository::new_user_pool(&pool);
  let user_stats_pool = Repository::new_user_stats_pool(&pool);
  let orbits = Repository::new_orbit_pool(&pool);
  let orbit_moderators = Repository::new_orbit_moderator_pool(&pool);
  let user_orbits = Repository::new_user_orbit_pool(&pool);

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
      .app_data(web::Data::new(app_pool.clone()))
      .app_data(web::Data::new(comment_pool.clone()))
      .app_data(web::Data::new(event_pool.clone()))
      .app_data(web::Data::new(follow_pool.clone()))
      .app_data(web::Data::new(job_pool.clone()))
      .app_data(web::Data::new(like_pool.clone()))
      .app_data(web::Data::new(post_pool.clone()))
      .app_data(web::Data::new(post_attachment_pool.clone()))
      .app_data(web::Data::new(session_pool.clone()))
      .app_data(web::Data::new(user_pool.clone()))
      .app_data(web::Data::new(user_stats_pool.clone()))
      .app_data(web::Data::new(orbits.clone()))
      .app_data(web::Data::new(orbit_moderators.clone()))
      .app_data(web::Data::new(user_orbits.clone()))
      .app_data(web::Data::new(Cdn::new()))
      .app_data(web::Data::new(Queue::new()))
      .service(
        web::resource("/api/users/{handle}")
          .name("get_user_by_id")
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_user_profile),
          )
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_user))
          .route(web::get().to(api_get_user_profile)),
      )
      .service(
        web::resource("/api/users/{handle}/feed")
          .name("get_user_public_feed")
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_federated_user_posts),
          )
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_federated_user_posts))
          .route(web::get().to(api_get_user_posts)),
      )
      .service(
        web::resource("/api/users/{handle}/likes")
          .name("get_user_public_likes_feed")
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_federated_user_liked_posts),
          )
          .route(
            web::get()
              .guard(HTML_GUARD)
              .to(api_redirect_to_federated_user_liked_posts),
          )
          .route(web::get().to(api_get_user_liked_posts)),
      )
      .service(
        web::resource("/api/users/{user_handle}/follows")
          .name("user_follows")
          .route(web::post().to(api_create_follow))
          .route(web::delete().to(api_delete_follow)),
      )
      .service(
        web::resource("/api/users/{user_handle}/followers")
          .name("user_followers")
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_user_followers),
          )
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_user_followers))
          .route(web::get().to(api_get_user_followers)),
      )
      .service(
        web::resource("/api/users/{user_handle}/following")
          .name("user_following")
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_user_following),
          )
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_user_following))
          .route(web::get().to(api_get_user_following)),
      )
      .service(
        web::resource("/api/users/{handle}/stats")
          .name("user_stats")
          .route(web::get().to(api_get_user_stats)),
      )
      .service(
        web::resource("/api/users/{handle}/orbits")
          .name("user_orbits")
          .route(web::get().to(api_get_user_orbits)),
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
        web::resource("/api/orbits/{orbit_shortcode}/feed")
          .name("orbit_feed")
          .route(web::get().to(api_get_orbit_feed)),
      )
      .service(
        web::resource("/api/feed/{post_id}")
          .name("post")
          .route(web::get().guard(ACTIVITYPUB_ACCEPT_GUARD).to(api_activitypub_get_post))
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_post))
          .route(web::get().to(api_get_post))
          .route(web::post().to(api_upload_post_image)),
      )
      .service(
        web::resource("/api/users/{user_handle}/feed/{post_id}")
          .name("user_post")
          .route(web::get().to(api_get_user_post)),
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
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_comments),
          )
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_post_comments))
          .route(web::get().to(api_get_comments))
          .route(web::post().to(api_create_comment)),
      )
      .service(
        web::resource("/api/feed/{post_id}/boost")
          .name("post_boosts")
          .route(web::post().to(api_boost_post))
          .route(web::delete().to(api_unboost_post)),
      )
      .service(
        web::resource("/api/feed/{post_id}/comments/{comment_id}/likes")
          .name("post_comment_likes")
          .route(web::post().to(api_create_comment_like))
          .route(web::delete().to(api_delete_comment_like)),
      )
      .service(
        web::resource("/api/feed/{post_id}/comments/{comment_id}")
          .name("comment")
          .route(
            web::get()
              .guard(ACTIVITYPUB_ACCEPT_GUARD)
              .to(api_activitypub_get_comment),
          )
          .route(web::get().guard(HTML_GUARD).to(api_redirect_to_post_comment))
          .route(web::get().to(api_get_comment))
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
        web::resource("/api/apps")
          .name("apps")
          .route(web::post().to(api_create_app)),
      )
      .service(
        web::resource("/api/orbits")
          .name("orbits")
          .route(web::get().to(api_get_orbits))
          .route(web::post().to(api_create_orbit)),
      )
      .service(
        web::resource("/api/orbits/{orbit_name}")
          .name("orbit_named")
          .route(web::get().to(api_get_orbit_named)),
      )
      .service(
        web::resource("/api/orbit/{orbit_id}")
          .name("orbit")
          .route(web::get().to(api_get_orbit))
          .route(web::patch().to(api_update_orbit))
          .route(web::delete().to(api_delete_orbit)),
      )
      .service(
        web::resource("/api/orbit/{orbit_id}/assets")
          .name("orbit_assets")
          .route(web::post().to(api_update_orbit_assets)),
      )
      .service(
        web::resource("/api/orbit/{orbit_id}/join")
          .name("orbit_join")
          .route(web::post().to(api_join_orbit)),
      )
      .service(
        web::resource("/api/orbit/{orbit_id}/leave")
          .name("orbit_leave")
          .route(web::post().to(api_leave_orbit)),
      )
      .service(
        web::resource("/api/orbit/{orbit_id}/moderators")
          .name("orbit_moderators")
          .route(web::get().to(api_get_orbit_moderators))
          .route(web::post().to(api_create_orbit_moderator))
          .route(web::patch().to(api_update_orbit_moderator))
          .route(web::delete().to(api_delete_orbit_moderator)),
      )
      .service(
        web::resource("/api/federate/activitypub/inbox/{user_handle}")
          .name("federate_activitypub")
          .route(web::post().to(api_activitypub_federate_user_inbox)),
      )
      .service(
        web::resource("/api/federate/activitypub/shared-inbox")
          .name("federate_activitypub")
          .route(web::post().to(api_activitypub_federate_shared_inbox)),
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
        web::resource("/.well-known/nodeinfo")
          .name("nodeinfo")
          .route(web::get().to(api_get_nodeinfo)),
      )
      .service(
        web::resource("/.well-known/host-meta")
          .name("nodeinfo")
          .route(web::get().to(api_get_host_meta)),
      )
      .service(
        web::resource("/api/nodeinfo/2.1")
          .name("nodeinfo")
          .route(web::get().to(api_get_nodeinfo_2_1)),
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
