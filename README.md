# rust-parceljs

Cargo.toml

```toml
[dependencies]
includedir = { git = "https://github.com/SpiralP/includedir.git" }
parceljs = { git = "https://github.com/SpiralP/rust-parceljs.git", branch = "next" }
phf = "0.8.0"

[build-dependencies]
parceljs-builder = { path = "../builder" }
```

build.rs

```rust
fn main() {
  parceljs_builder::build();
}
```

main.rs

```rust
include!(concat!(env!("OUT_DIR"), "/web_files.rs"));

fn main() {
  println!(
    "{}",
    String::from_utf8_lossy(&parceljs::get_file(&WEB_FILES, "index.html").unwrap())
  );
}
```
