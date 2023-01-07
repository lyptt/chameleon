#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]
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
mod scheduled_tasks;
mod settings;
mod work_queue;
mod worker_internal;

use crate::worker_internal::services::{DB, QUEUE};
use aws::clients::AWSClient;
use cdn::cdn_store::Cdn;
use env_logger::WriteStyle;
use log::error;
use log::LevelFilter;
use rabbitmq::clients::RabbitMQClient;
use scheduled_tasks::scheduler::JobScheduler;
use settings::SETTINGS;

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

  let _ = match SETTINGS.queue.schedule_jobs {
    true => Some(JobScheduler::new()),
    false => None,
  };

  loop {
    match QUEUE.receive_jobs(&cdn, &QUEUE, &DB).await {
      Ok(_) => {}
      Err(err) => error!("{}", err.to_string()),
    }
  }
}
