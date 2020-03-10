#[cfg(feature = "with_actix")]
include!(concat!(env!("OUT_DIR"), "/parceljs.rs"));

#[cfg(feature = "with_actix")]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  use actix_web::{App, HttpServer};

  println!("try http://127.0.0.1:8080/");
  HttpServer::new(|| App::new().route("/*", PARCELJS.as_route()))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(not(feature = "with_actix"))]
fn main() {
  panic!("run with --features with_actix");
}
