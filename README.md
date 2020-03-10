# rust-parceljs

package.json

```json
  "scripts": {
    "build": "parcel build web/index.html"
  },
  "devDependencies": {
    "parcel-bundler": "^1.12.3"
  }
```

Cargo.toml

```toml
[dependencies]
# optional features: actix, warp
parceljs = { git = "https://github.com/SpiralP/rust-parceljs.git" }

[build-dependencies]
parceljs-builder = { git = "https://github.com/SpiralP/rust-parceljs.git" }
```

build.rs

```rust
fn main() {
  parceljs_builder::build();
}
```

main.rs

```rust
include!(concat!(env!("OUT_DIR"), "/parceljs.rs"));

fn main() {
  println!(
    "{}",
    String::from_utf8_lossy(&PARCELJS.get_file("index.html").unwrap())
  );
}
```

main.rs with [warp](https://github.com/seanmonstar/warp) feature enabled

```rust
include!(concat!(env!("OUT_DIR"), "/parceljs.rs"));

#[tokio::main]
async fn main() {
  println!("try http://127.0.0.1:3030/");
  warp::serve(PARCELJS.as_filter())
    .run(([127, 0, 0, 1], 3030))
    .await;
}
```

main.rs with [actix](https://github.com/actix/actix-web) feature enabled

```rust
include!(concat!(env!("OUT_DIR"), "/parceljs.rs"));

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  use actix_web::{App, HttpServer};

  println!("try http://127.0.0.1:8080/");
  HttpServer::new(|| App::new().route("/*", PARCELJS.as_route()))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
