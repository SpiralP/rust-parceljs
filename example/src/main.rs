include!(concat!(env!("OUT_DIR"), "/parceljs.rs"));

fn main() {
  let data = PARCELJS.get_file("index.html").unwrap();
  println!("{}", String::from_utf8_lossy(&data));
}
