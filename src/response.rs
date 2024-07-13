use chrono::Utc;
use http::StatusCode;
use std::collections::HashMap;

pub struct Response {
  pub status: u16,
  status_modificated: bool,
  pub headers: HashMap<String, String>,
  pub body: String,
}

impl Response {
  ///
  /// Creates new Response
  ///
  /// Initial headers `X-Powered-By`, `Connection`, `Keep-Alive`, `Server` and `Date` are included here
  ///
  pub fn new() -> Response {
    let i_headers = HashMap::from([
      (String::from("X-Powered-By"), String::from("Saturn")),
      (String::from("Connection"), String::from("keep-alive")),
      (String::from("Keep-Alive"), String::from("timeout=5")),
      (String::from("Server"), String::from("Saturn")),
      (
        String::from("Date"),
        Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
      ),
    ]);

    Response {
      status: 204,
      status_modificated: false,
      headers: i_headers,
      body: String::new(),
    }
  }

  /// Set status of Response
  pub fn status(&mut self, status: u16) -> &mut Self {
    self.status_modificated = true;
    self.status = status;
    self
  }

  /// Set a header for the Response
  pub fn set_header(&mut self, header: String, value: String) -> &mut Self {
    self.headers.insert(header, value);
    self
  }

  ///
  /// Send data to the client as text
  ///
  /// Header `Content-Type` is set to `text/html; charset=utf-8`
  /// Should be the last method used on the Response builder
  ///
  pub fn send(&mut self, response: String) {
    if !self.status_modificated {
      self.status = 200;
    }
    self.headers.insert(
      String::from("Content-Type"),
      String::from("text/html; charset=utf-8"),
    );
    self.body = response;
  }

  ///
  /// Send data to the client as json
  ///
  /// Header `Content-Type` is set to `application/json`
  /// Should be the last method used on the Response builder
  ///
  pub fn json(&mut self, response: String) {
    if !self.status_modificated {
      self.status = 200;
    }
    self.headers.insert(
      String::from("Content-Type"),
      String::from("application/json"),
    );
    self.body = response;
  }

  /// Generate ETag
  fn generate_etag(&self) -> String {
    let digest = md5::compute(self.body.as_bytes());
    let hash_hex = format!("{:x}", digest);
    hash_hex
  }

  ///
  /// Build the head
  ///
  /// Final headers `Content-Length` and `ETag` are included here
  ///
  pub fn build_head(&mut self) -> String {
    let mut response = format!(
      "HTTP/1.1 {}\r\n",
      StatusCode::from_u16(self.status).unwrap_or_else(|e| {
        println!("Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
      })
    );

    let mut f_headers = HashMap::new();
    f_headers.insert(String::from("Content-Length"), self.body.len().to_string());
    f_headers.insert(
      String::from("ETag"),
      format!("W/\"{}\"", self.generate_etag()),
    );
    self.headers.extend(f_headers);

    let mut sorted_headers: Vec<_> = self.headers.iter().collect();
    sorted_headers.sort_by_key(|(header, _)| *header);

    for (header, value) in sorted_headers {
      response.push_str(&format!("{}: {}\r\n", header, value));
    }

    response.push_str("\r\n");

    response
  }

  ///
  /// Build the response
  ///
  /// `head` + `body`
  ///
  pub fn build_response(&mut self) -> String {
    self.build_head() + &self.body
  }
}
