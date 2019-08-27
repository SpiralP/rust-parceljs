use std::{borrow, io};

include!(concat!(env!("OUT_DIR"), "/web_files.rs"));

pub fn get_file(file_path: &'static str) -> Result<borrow::Cow<'static, [u8]>, io::Error> {
  WEB_FILES.get(&format!("dist/{}", file_path))
}
