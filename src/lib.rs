#![allow(clippy::unreadable_literal)]

#[cfg(feature = "warp")]
pub mod warp;

#[cfg(feature = "actix")]
pub mod actix;

use std::{borrow, io};

include!(concat!(env!("OUT_DIR"), "/web_files.rs"));

pub fn get_file(mut file_path: &str) -> Result<borrow::Cow<'static, [u8]>, io::Error> {
  if let Some(c) = file_path.chars().nth(0) {
    if c == '/' {
      file_path = &file_path[1..];
    }
  }
  if file_path.is_empty() {
    file_path = "index.html";
  }

  WEB_FILES.get(&format!("dist/{}", file_path))
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
