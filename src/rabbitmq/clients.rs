use lapin::{
  options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
  types::{AMQPValue, FieldTable},
  Connection, ConnectionProperties, ExchangeKind,
};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

use crate::settings::{AppQueueBackend, SETTINGS};

lazy_static! {
  pub static ref RABBITMQ_CLIENT: OnceCell<lapin::Connection> = OnceCell::new();
  pub static ref RABBITMQ_WORK_CHANNEL: OnceCell<lapin::Channel> = OnceCell::new();
  pub static ref RABBITMQ_WORK_QUEUE: OnceCell<lapin::Queue> = OnceCell::new();
  pub static ref RABBITMQ_DL_QUEUE: OnceCell<lapin::Queue> = OnceCell::new();
}

pub struct RabbitMQClient {}

impl RabbitMQClient {
  pub async fn create_rabbitmq_client() {
    if SETTINGS.queue.queue_backend != AppQueueBackend::RabbitMQ {
      return;
    }

    let url = SETTINGS
      .queue
      .url
      .clone()
      .unwrap_or_else(|| "amqp://127.0.0.1:5672".to_string());

    let conn = Connection::connect(&url, ConnectionProperties::default())
      .await
      .unwrap();

    let work_channel = conn.create_channel().await.unwrap();

    let exchg_declare_options = ExchangeDeclareOptions {
      passive: false,
      durable: true,
      auto_delete: false,
      internal: false,
      nowait: false,
    };

    let dl_exchg_name = format!("exchg_{}", SETTINGS.queue.work_deadletter_queue);
    let exchg_name = format!("exchg_{}", SETTINGS.queue.work_queue);

    work_channel
      .exchange_declare(
        &dl_exchg_name,
        ExchangeKind::Direct,
        exchg_declare_options,
        FieldTable::default(),
      )
      .await
      .unwrap();

    work_channel
      .exchange_declare(
        &exchg_name,
        ExchangeKind::Direct,
        exchg_declare_options,
        FieldTable::default(),
      )
      .await
      .unwrap();

    let queue_options = QueueDeclareOptions {
      durable: true,
      exclusive: false,
      auto_delete: false,
      ..Default::default()
    };

    let mut queue_args = FieldTable::default();
    queue_args.insert(
      "x-dead-letter-exchange".into(),
      AMQPValue::LongString(dl_exchg_name.clone().into()),
    );

    let work_queue = work_channel
      .queue_declare(&SETTINGS.queue.work_queue, queue_options, queue_args)
      .await
      .unwrap();

    let dl_queue = work_channel
      .queue_declare(
        &SETTINGS.queue.work_deadletter_queue,
        queue_options,
        FieldTable::default(),
      )
      .await
      .unwrap();

    work_channel
      .queue_bind(
        &SETTINGS.queue.work_deadletter_queue,
        &dl_exchg_name,
        "",
        QueueBindOptions::default(),
        FieldTable::default(),
      )
      .await
      .unwrap();

    work_channel
      .queue_bind(
        &SETTINGS.queue.work_queue,
        &exchg_name,
        "",
        QueueBindOptions::default(),
        FieldTable::default(),
      )
      .await
      .unwrap();

    RABBITMQ_CLIENT.set(conn).unwrap();
    RABBITMQ_WORK_CHANNEL.set(work_channel).unwrap();
    RABBITMQ_WORK_QUEUE.set(work_queue).unwrap();
    RABBITMQ_DL_QUEUE.set(dl_queue).unwrap();
  }
}
