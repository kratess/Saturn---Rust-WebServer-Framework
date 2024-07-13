use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use thiserror::Error;

lazy_static! {
  static ref PATH_REGEX: Regex = Regex::new(r"^/.*$").unwrap();
}

#[derive(Debug, Error)]
pub enum HttpPathError {
  #[error("No HTTP path was provided")]
  NoPath,
  #[error("Unsupported HTTP path: {0}")]
  UnsupportedPath(String),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct HttpPath {
  path: String,
}

impl HttpPath {
  pub fn from_str(s: &str) -> Option<Self> {
    if PATH_REGEX.is_match(s) {
      Some(HttpPath {
        path: s.to_string(),
      })
    } else {
      None
    }
  }

  pub fn is_valid(s: &str) -> bool {
    PATH_REGEX.is_match(s)
  }

  pub fn get(s: &str) -> Result<Self, HttpPathError> {
    if s.is_empty() {
      return Err(HttpPathError::NoPath);
    }

    match HttpPath::from_str(s) {
      Some(path) => Ok(path),
      None => Err(HttpPathError::UnsupportedPath(s.to_string())),
    }
  }
}

impl fmt::Display for HttpPath {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.path)
  }
}