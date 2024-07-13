use crate::request;
use crate::request::RequestError;
use crate::response;

use crate::http::method::*;
use crate::http::path::*;
use crate::http::version::*;
use request::Request;
use response::Response;
use std::{
  borrow::Borrow,
  collections::HashMap,
  io::{BufRead, BufReader, Read, Write},
  net::{TcpListener, TcpStream},
  str::FromStr,
  sync::Arc,
  thread,
  time::Duration,
};
use thiserror::Error;

pub type Handler = fn(res: &mut Response);

pub struct WebServer {
  address: String,
  routes: HashMap<HttpPath, HashMap<HttpMethod, Handler>>,
}

impl WebServer {
  pub fn new(address: String) -> Self {
    WebServer {
      address,
      routes: HashMap::new(),
    }
  }

  fn add_route(&mut self, method: HttpMethod, path: HttpPath, handler: Handler) {
    let route_entry = self.routes.entry(path).or_insert_with(|| HashMap::new());
    route_entry.insert(method, handler);
  }

  pub fn get(&mut self, path: HttpPath, handler: Handler) {
    self.add_route(HttpMethod::Get, path, handler);
  }

  pub fn post(&mut self, path: HttpPath, handler: Handler) {
    self.add_route(HttpMethod::Post, path, handler);
  }

  pub fn start(self) {
    let listener = TcpListener::bind(&self.address).unwrap();
    let arc_self = Arc::new(self);

    for stream in listener.incoming() {
      let mut stream = stream.unwrap();
      /*stream
      .set_read_timeout(Some(Duration::from_secs(5)))
      .unwrap();*/
      let arc_self_clone = arc_self.clone();

      thread::spawn(
        move || match arc_self_clone.handle_connection(stream.try_clone().unwrap()) {
          Ok(_) => {}
          Err(e) => {
            println!("Error {}", e);
            arc_self_clone.bad_request(&mut stream, Some(e.to_string()));
          }
        },
      );
    }
  }

  fn handle_connection(&self, mut stream: TcpStream) -> Result<(), RequestError> {
    let req: Request = Request::new(&stream)?;

    println!("{:?}", req);

    // Check if path has any handlers
    let path_handlers = match self.routes.get(&req.path) {
      Some(map) => map,
      None => {
        self.not_found(&mut stream);
        return Ok(());
      }
    };

    // Find the right handler
    let handler: Option<&Handler> = path_handlers.get(&req.method);

    match req.method {
      HttpMethod::Head => match path_handlers.get(&HttpMethod::Get) {
        Some(_) => {
          let mut res = Response::new();
          if let Some(handler) = handler {
            handler(&mut res);
          }
          self.generic(&mut stream, res.build_head());
        }
        None => self.method_not_allowed(&mut stream),
      },
      HttpMethod::Options => {
        let mut res = Response::new();
        let mut allowed_methods = path_handlers
          .keys()
          .map(|k| k.to_string())
          .collect::<Vec<String>>();
        if !allowed_methods.contains(&HttpMethod::Options.to_string()) {
          allowed_methods.push(HttpMethod::Options.to_string());
        }
        if allowed_methods.contains(&HttpMethod::Get.to_string())
          && !allowed_methods.contains(&HttpMethod::Head.to_string())
        {
          allowed_methods.push(HttpMethod::Head.to_string());
        }
        res
          .status(204)
          .set_header(String::from("Allow"), allowed_methods.join(", "));
        if let Some(handler) = handler {
          handler(&mut res);
        }
        self.generic(&mut stream, res.build_head());
      }
      _ => {
        let mut res = Response::new();
        if let Some(handler) = handler {
          handler(&mut res);
        }
        self.generic(&mut stream, res.build_response());
      }
    }

    Ok(())
  }

  fn generic(&self, stream: &mut TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }

  fn not_found(&self, stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }

  fn method_not_allowed(&self, stream: &mut TcpStream) {
    let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }

  fn bad_request(&self, stream: &mut TcpStream, body: Option<String>) {
    let response = format!(
      "HTTP/1.1 400 BAD REQUEST\r\n\r\n{}",
      body.unwrap_or(String::new())
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }
}
