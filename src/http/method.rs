use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpMethodError {
  #[error("No HTTP method was provided")]
  NoMethod,
  #[error("Unsupported HTTP method: {0}")]
  UnsupportedMethod(String),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum HttpMethod {
  Options,
  Head,
  Get,
  Post,
}

impl HttpMethod {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "OPTIONS" => Some(HttpMethod::Options),
      "HEAD" => Some(HttpMethod::Head),
      "GET" => Some(HttpMethod::Get),
      "POST" => Some(HttpMethod::Post),
      _ => None,
    }
  }

  pub fn is_valid(s: &str) -> bool {
    match s {
      "OPTIONS" | "HEAD" | "GET" | "POST" => true,
      _ => false,
    }
  }

  pub fn get(s: &str) -> Result<Self, HttpMethodError> {
    if s.is_empty() {
      return Err(HttpMethodError::NoMethod);
    }

    match HttpMethod::from_str(s) {
      Some(method) => Ok(method),
      None => Err(HttpMethodError::UnsupportedMethod(s.to_string())),
    }
  }
}

impl fmt::Display for HttpMethod {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      HttpMethod::Options => write!(f, "OPTIONS"),
      HttpMethod::Head => write!(f, "HEAD"),
      HttpMethod::Get => write!(f, "GET"),
      HttpMethod::Post => write!(f, "POST"),
    }
  }
}
