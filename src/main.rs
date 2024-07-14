mod http;

mod request;
mod response;
mod server;

use request::Request;
use response::Response;
use server::WebServer;

fn main() {
  let mut server: WebServer = WebServer::new("127.0.0.1:7878");

  server.get("/", get_main);
  server.post("/", post_main);

  server.start();
}

fn get_main(req: &mut Request, res: &mut Response) {
  println!("{:?}", req.params.get("sad"));
  res.send(String::from("Hello World!"));
}

fn post_main(req: &mut Request, res: &mut Response) {
  res.json(String::from("{\"test\": \"hello world\"}"));
}
