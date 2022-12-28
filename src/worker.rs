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

use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use db::repositories::Repositories;
use deadpool::Runtime;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::RecyclingMethod;
use env_logger::WriteStyle;
use log::error;
use log::LevelFilter;
use rabbitmq::clients::RabbitMQClient;
use settings::SETTINGS;
use std::time::Duration;
use tokio_postgres::NoTls;
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
  RabbitMQClient::create_rabbitmq_client().await;

  let cdn = Cdn::new();

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

  let repositories = Repositories::new(&pool);

  let queue = Queue::new();

  loop {
    match queue.receive_jobs(&cdn, &queue, &repositories).await {
      Ok(_) => {}
      Err(err) => error!("{}", err.to_string()),
    }
  }
}
