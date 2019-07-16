extern crate actix_web;
extern crate json;
use actix_web::{http, server, App, HttpRequest, HttpResponse};

fn notebooks(_req: &HttpRequest) -> HttpResponse {
  let parsed = json::parse(
    r#"
      [
        {
          "id": 1,
          "content": "This is my first notebook"
        },
        {
          "id": 2,
          "content": "This is my second notebook"
        }
      ]
    "#,
  )
  .unwrap();
  let stringified = json::stringify(parsed);

  HttpResponse::Ok()
    .header(http::header::CONTENT_TYPE, "application/json")
    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .body(stringified)
}

fn notebook(_req: &HttpRequest) -> HttpResponse {
  let parsed = json::parse(
    r#"
      {
        "id": 1,
        "content": "This is the first notebook"
      }
    "#,
  )
  .unwrap();
  let stringified = json::stringify(parsed);
  HttpResponse::Ok()
    .header(http::header::CONTENT_TYPE, "application/json")
    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .body(stringified)
}

fn main() {
  server::new(|| {
    App::new()
      .resource("/notebooks", |r| r.f(notebooks))
      .resource("/1", |r| r.f(notebook))
  })
  .bind("127.0.0.1:8088")
  .unwrap()
  .run();
}
