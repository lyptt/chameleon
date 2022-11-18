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
mod settings;
mod work_queue;

use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use env_logger::WriteStyle;
use log::error;
use log::LevelFilter;
use settings::SETTINGS;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use work_queue::queue::Queue;

#[tokio::main]
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

  AWSClient::create_s3_client().await;
  AWSClient::create_sqs_client().await;

  let cdn = Cdn::new();

  let pool = PgPoolOptions::new()
    .max_connections(SETTINGS.database.max_connections)
    .idle_timeout(Duration::from_secs(SETTINGS.database.idle_timeout.into()))
    .acquire_timeout(Duration::from_secs(SETTINGS.database.connection_timeout.into()))
    .connect(&SETTINGS.database.url)
    .await
    .unwrap();

  let queue = Queue::new();

  loop {
    match queue.receive_jobs(pool.clone(), &cdn).await {
      Ok(_) => {}
      Err(err) => error!("{}", err.to_string()),
    }
  }
}
