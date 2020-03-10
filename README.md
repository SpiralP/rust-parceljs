# rust-parceljs

Cargo.toml

```toml
[dependencies]
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

package.json

```json
  "scripts": {
    "build": "parcel build web/index.html"
  },
  "devDependencies": {
    "parcel-bundler": "^1.12.3"
  }
```
