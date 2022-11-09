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

#[derive(Clone, Debug, Deserialize, EnumString, Display)]
pub enum AppCdnStore {
  // TODO: Add support for other CDNs (S3, Blob Storage, etc)
  Local,
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
  pub url: String,
  pub max_connections: u32,
  pub idle_timeout: u32,
  pub connection_timeout: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cdn {
  pub file_store: AppCdnStore,
  pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  pub server: Server,
  pub database: Database,
  pub log: Log,
  pub env: AppEnv,
  pub cdn: Cdn,
}

fn get_cwd() -> String {
  match std::env::current_dir() {
    Ok(path) => path.into_os_string().into_string().unwrap_or(".".to_string()),
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
        url: "postgresql://root:root@127.0.0.1:5432/chameleon".to_string(),
        max_connections: 1,
        idle_timeout: 30,
        connection_timeout: 30,
      },
      server: Server {
        url: "0.0.0.0".to_string(),
        port: 8080,
        fqdn: "http://0.0.0.0:8080".to_string(),
        api_fqdn: "http://0.0.0.0:8080/api".to_string(),
      },
      cdn: Cdn {
        file_store: AppCdnStore::Local,
        path: get_cwd(),
      },
    }
  }

  fn new() -> Self {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let settings = Config::builder()
      .add_source(config::File::with_name("config/default"))
      .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
      .add_source(config::File::with_name("config/local").required(false))
      .add_source(config::Environment::with_prefix("chameleon"))
      .build();

    return match settings {
      Ok(settings) => match settings.try_deserialize() {
        Ok(settings) => settings,
        Err(err) => {
          eprintln!("{}", err);
          Settings::default()
        }
      },
      Err(err) => {
        eprintln!("{}", err);
        return Settings::default();
      }
    };
  }
}

lazy_static! {
  pub static ref SETTINGS: Settings = Settings::new();
}
