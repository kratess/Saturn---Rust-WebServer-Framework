use serde::Deserialize;
use serde_json::{self, Value};
use std::{
  collections::HashMap,
  io::{BufRead, BufReader, Read},
  net::TcpStream,
};

use thiserror::Error;

use crate::http::{
  method::{HttpMethod, HttpMethodError},
  path::{HttpPath, HttpPathError},
  version::{HttpVersion, HttpVersionError},
};

#[derive(Debug, Error)]
pub enum RequestError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  HttpMethodError(#[from] HttpMethodError),
  #[error(transparent)]
  HttpPathError(#[from] HttpPathError),
  #[error(transparent)]
  HttpVersionError(#[from] HttpVersionError),
}

#[derive(Debug)]
pub struct Request {
  pub method: HttpMethod,
  pub path: HttpPath,
  pub http_version: HttpVersion,
  pub headers: HashMap<String, String>,
  pub params: HashMap<String, String>,
  pub body: Value,
}

impl Request {
  pub fn new(stream: &TcpStream) -> Result<Request, RequestError> {
    let mut buf_reader = BufReader::new(stream);

    // Read the first line to get method and path
    let mut first_line = String::new();
    buf_reader.read_line(&mut first_line)?;
    let mut parts = first_line.trim().splitn(3, " ");

    // Extract method
    let method = match parts.next() {
      Some(m) => HttpMethod::get(m)?,
      None => return Err(RequestError::HttpMethodError(HttpMethodError::NoMethod)),
    };

    let full_path = parts.next();
    if full_path.is_none() {
      return Err(RequestError::HttpPathError(HttpPathError::NoPath));
    }
    let mut path_splitted = full_path.unwrap().splitn(2, "?");

    // Extract path
    let path = match path_splitted.next() {
      Some(m) => HttpPath::get(m)?,
      None => return Err(RequestError::HttpPathError(HttpPathError::NoPath)),
    };

    // Extract http version
    let http_version = match parts.next() {
      Some(m) => HttpVersion::get(m)?,
      None => return Err(RequestError::HttpVersionError(HttpVersionError::NoVersion)),
    };

    let mut params = HashMap::new();

    if method == HttpMethod::Get {
      if let Some(d) = path_splitted.next() {
        params = get_params_from_query(d.to_owned());
      }
    }

    // Read headers until an empty line (end of headers)
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in buf_reader.by_ref().lines() {
      let line = line?;
      if line.trim().is_empty() {
        break;
      }
      let mut line_splitted = line.splitn(2, ":");
      if let (Some(name), Some(value)) = (line_splitted.next(), line_splitted.next()) {
        headers.insert(name.trim().to_owned(), value.trim().to_owned());
      }
    }

    // Find and read the body based on Content-Length header
    let content_length = headers
      .get("Content-Length")
      .and_then(|value| value.parse::<usize>().ok())
      .unwrap_or(0);

    let mut b: Value = Value::Null;

    // Read the body if Content-Length is greater than 0
    if content_length > 0 {
      let mut body = vec![0; content_length];
      buf_reader.read_exact(&mut body)?;
      let body_str = String::from_utf8_lossy(&body);
      b = serde_json::from_str(&body_str).expect("Body was not well-formatted (JSON)");
    }

    Ok(Request {
      method,
      path,
      http_version,
      headers,
      params,
      body: b,
    })
  }
}

fn get_params_from_query(query: String) -> HashMap<String, String> {
  let mut map = HashMap::new();

  for pair in query.split('&') {
    let mut iter = pair.split('=');
    if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
      map.insert(key.to_string(), value.to_string());
    }
  }

  map
}
