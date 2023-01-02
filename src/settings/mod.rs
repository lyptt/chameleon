use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;
use strum::{Display, EnumString};

#[derive(Clone, Debug, Deserialize, EnumString, Display)]
pub enum AppEnv {
  Development,
  Testing,
  Production,
}

#[derive(Clone, Debug, Deserialize, EnumString, Display)]
pub enum AppLogLevel {
  Critical,
  Normal,
  Debug,
}

#[derive(Clone, Debug, Deserialize, EnumString, Display, PartialEq, Eq)]
pub enum AppCdnStore {
  Local,
  S3,
}

#[derive(Clone, Debug, Deserialize, EnumString, Display, PartialEq, Eq)]
pub enum AppQueueBackend {
  Noop,
  RabbitMQ,
  Sqs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
  pub level: AppLogLevel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
  pub port: u16,
  pub url: String,
  pub fqdn: String,
  pub api_fqdn: String,
  pub api_root_fqdn: String,
  pub cdn_fqdn: String,
  pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
  pub host: String,
  pub port: u16,
  pub database: String,
  pub username: String,
  pub password: String,
  pub max_connections: usize,
  pub idle_timeout: u32,
  pub connection_timeout: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudCredentials {
  pub access_key: String,
  pub secret_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cdn {
  pub file_store: AppCdnStore,
  pub path: String,
  pub container: Option<String>,
  pub credentials: Option<CloudCredentials>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Queue {
  pub queue_backend: AppQueueBackend,
  pub work_queue: String,
  pub url: Option<String>,
  pub work_deadletter_queue: String,
  pub credentials: Option<CloudCredentials>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
  pub imagemagick_exe_path: String,
  pub secure: bool,
  pub verify_external_https_certificates: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  pub server: Server,
  pub database: Database,
  pub log: Log,
  pub env: AppEnv,
  pub cdn: Cdn,
  pub queue: Queue,
  pub app: Application,
}

fn get_cwd() -> String {
  match std::env::current_dir() {
    Ok(path) => path.into_os_string().into_string().unwrap_or_else(|_| ".".to_string()),
    Err(_) => ".".to_string(),
  }
}

impl Settings {
  fn default() -> Self {
    Settings {
      env: AppEnv::Development,
      log: Log {
        level: AppLogLevel::Debug,
      },
      database: Database {
        host: "127.0.0.1".to_string(),
        port: 5432,
        database: "orbit".to_string(),
        username: "root".to_string(),
        password: "root".to_string(),
        max_connections: 1,
        idle_timeout: 30,
        connection_timeout: 30,
      },
      server: Server {
        url: "0.0.0.0".to_string(),
        port: 8080,
        fqdn: "http://0.0.0.0:8080".to_string(),
        api_fqdn: "http://0.0.0.0:8080/api".to_string(),
        api_root_fqdn: "http://0.0.0.0:8080".to_string(),
        cdn_fqdn: "http://0.0.0.0:8080".to_string(),
        jwt_secret: "change-me".to_string(),
      },
      cdn: Cdn {
        file_store: AppCdnStore::Local,
        path: get_cwd(),
        container: None,
        credentials: None,
      },
      queue: Queue {
        queue_backend: AppQueueBackend::RabbitMQ,
        url: Some("amqp://127.0.0.1:5672".to_string()),
        work_queue: "work_q".to_string(),
        work_deadletter_queue: "work_dq".to_string(),
        credentials: None,
      },
      app: Application {
        imagemagick_exe_path: "convert".to_string(),
        secure: false,
        verify_external_https_certificates: false,
      },
    }
  }

  fn new() -> Self {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let mut builder = Config::builder()
      .add_source(config::File::with_name("config/default"))
      .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false));

    if run_mode != "production" {
      builder = builder.add_source(config::File::with_name("config/local").required(false));
    }

    let settings = builder.add_source(config::Environment::with_prefix("orbit")).build();

    match settings {
      Ok(settings) => match settings.try_deserialize() {
        Ok(settings) => settings,
        Err(err) => {
          eprintln!("{}", err);
          Settings::default()
        }
      },
      Err(err) => {
        eprintln!("{}", err);
        Settings::default()
      }
    }
  }
}

lazy_static! {
  pub static ref SETTINGS: Settings = Settings::new();
}
