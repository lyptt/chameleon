use crate::db::repositories::Repositories;
use crate::settings::SETTINGS;
use deadpool::Runtime;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::RecyclingMethod;
use lazy_static::lazy_static;
use std::time::Duration;
use tokio_postgres::NoTls;

use crate::work_queue::queue::Queue;

lazy_static! {
  pub static ref QUEUE: Queue = Queue::new();
  pub static ref DB: Repositories = {
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

    Repositories::new(pool)
  };
}
