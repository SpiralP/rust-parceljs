# rust-parceljs

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
