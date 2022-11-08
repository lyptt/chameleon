use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, fmt};

#[derive(Clone, Debug, Deserialize)]
pub enum AppEnv {
  Development,
  Testing,
  Production,
}

impl fmt::Display for AppEnv {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AppEnv::Development => write!(f, "Development"),
      AppEnv::Testing => write!(f, "Testing"),
      AppEnv::Production => write!(f, "Production"),
    }
  }
}

impl From<&str> for AppEnv {
  fn from(env: &str) -> Self {
    match env {
      "Testing" => AppEnv::Testing,
      "Production" => AppEnv::Production,
      _ => AppEnv::Development,
    }
  }
}

#[derive(Clone, Debug, Deserialize)]
pub enum AppLogLevel {
  Critical,
  Normal,
  Debug,
}

impl fmt::Display for AppLogLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AppLogLevel::Critical => write!(f, "Critical"),
      AppLogLevel::Normal => write!(f, "Normal"),
      AppLogLevel::Debug => write!(f, "Debug"),
    }
  }
}

impl From<&str> for AppLogLevel {
  fn from(env: &str) -> Self {
    match env {
      "Critical" => AppLogLevel::Critical,
      "Normal" => AppLogLevel::Normal,
      "Debug" => AppLogLevel::Debug,
      _ => AppLogLevel::Debug,
    }
  }
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
pub struct Settings {
  pub server: Server,
  pub database: Database,
  pub log: Log,
  pub env: AppEnv,
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
