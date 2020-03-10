#![allow(clippy::unreadable_literal)]

#[cfg(feature = "warp")]
pub mod warp;

#[cfg(feature = "actix")]
pub mod actix;

use std::{borrow::Cow, io};

pub fn get_file(
  web_files: &includedir::Files,
  mut file_path: &str,
) -> Result<Cow<'static, [u8]>, io::Error> {
  if let Some(c) = file_path.chars().next() {
    if c == '/' {
      file_path = &file_path[1..];
    }
  }
  if file_path.is_empty() {
    file_path = "index.html";
  }

  // TODO remove "dist" from includedir
  web_files.get(&format!("dist/{}", file_path))
}

pub fn get_content_type(file_path: &str) -> Option<String> {
  if file_path.ends_with(".css") {
    Some("text/css; charset=utf-8".to_string())
  } else if file_path.ends_with(".js") {
    Some("application/javascript; charset=utf-8".to_string())
  } else if file_path.ends_with(".html") {
    Some("text/html; charset=utf-8".to_string())
  } else {
    None
  }
}
