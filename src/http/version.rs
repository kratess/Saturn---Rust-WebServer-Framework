use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpVersionError {
  #[error("No HTTP version was provided")]
  NoVersion,
  #[error("Unsupported HTTP version: {0}")]
  UnsupportedVersion(String),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum HttpVersion {
  Http09,
  Http10,
  Http11,
  Http2,
  Http3,
}

impl HttpVersion {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "HTTP/0.9" => Some(HttpVersion::Http09),
      "HTTP/1.0" => Some(HttpVersion::Http10),
      "HTTP/1.1" => Some(HttpVersion::Http11),
      "HTTP/2" => Some(HttpVersion::Http2),
      "HTTP/3" => Some(HttpVersion::Http3),
      _ => None,
    }
  }

  pub fn is_valid(s: &str) -> bool {
    match s {
      "HTTP/0.9" | "HTTP/1.0" | "HTTP/1.1" | "HTTP/2" | "HTTP/3" => true,
      _ => false,
    }
  }

  pub fn get(s: &str) -> Result<Self, HttpVersionError> {
    if s.is_empty() {
      return Err(HttpVersionError::NoVersion);
    }

    match HttpVersion::from_str(s) {
      Some(method) => Ok(method),
      None => Err(HttpVersionError::UnsupportedVersion(s.to_string())),
    }
  }
}

impl fmt::Display for HttpVersion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      HttpVersion::Http09 => write!(f, "HTTP/0.9"),
      HttpVersion::Http10 => write!(f, "HTTP/1.0"),
      HttpVersion::Http11 => write!(f, "HTTP/1.1"),
      HttpVersion::Http2 => write!(f, "HTTP/2.0"),
      HttpVersion::Http3 => write!(f, "HTTP/3.0"),
    }
  }
}
