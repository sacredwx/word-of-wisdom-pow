use std::{fmt, error::Error};

#[derive(Debug, Clone)]
pub enum ServerError {
  Serialize,
  Deserialize,
  Communication,
  DataSelectionError,
}

impl Error for ServerError {}

impl fmt::Display for ServerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Self::Serialize => write!(f, "Serialization Error"),
        Self::Deserialize => write!(f, "Deserialization Error"),
        Self::Communication => write!(f, "Communication Error"),
        Self::DataSelectionError => write!(f, "Data Selection Error"),
      }
  }
}
