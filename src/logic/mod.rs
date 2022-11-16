use std::{error::Error, fmt::Display};

pub mod post;
pub mod user;

#[derive(Debug)]
pub enum LogicErr {
  // NotFound,
  DbError(String),
  UnauthorizedError,
  InternalError(String),
  InvalidOperation(String),
}

impl Display for LogicErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LogicErr::DbError(err) => f.write_fmt(format_args!("DbError: {}", err)),
      LogicErr::UnauthorizedError => f.write_fmt(format_args!("UnauthorizedError")),
      LogicErr::InternalError(err) => f.write_fmt(format_args!("InternalError: {}", err)),
      LogicErr::InvalidOperation(err) => f.write_fmt(format_args!("InvalidOperation: {}", err)),
    }
  }
}

impl Error for LogicErr {}
