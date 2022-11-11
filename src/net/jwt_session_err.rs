use std::error::Error;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Display)]
pub enum JwtSessionErr {
  InvalidDataErr,
  JwtError(jsonwebtoken::errors::Error),
}

impl Error for JwtSessionErr {}
