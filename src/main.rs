mod http;

mod request;
mod response;
mod server;

use http::path::HttpPath;
use response::Response;
use server::WebServer;

fn main() {
  let mut server: WebServer = WebServer::new("127.0.0.1:7878".to_string());

  server.get(HttpPath::from_str("/").unwrap(), |res: &mut Response| {
    res.send(String::from("Hello World!"));
  });

  server.post(HttpPath::from_str("/").unwrap(), |res: &mut Response| {
    res.json(String::from("{\"test\": \"hello world\"}"));
  });

  server.start();
}
