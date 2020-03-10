use super::{get_content_type, get_file};
use warp::{http::Response, path::FullPath, reply, reply::Reply};

pub struct ParceljsResponder {
  web_files: &'static includedir::Files,
  path: FullPath,
}

impl ParceljsResponder {
  pub fn new(web_files: &'static includedir::Files, path: FullPath) -> Self {
    Self { web_files, path }
  }
}

impl Reply for ParceljsResponder {
  fn into_response(self) -> reply::Response {
    let path = self.path.as_str();

    if let Ok(data) = get_file(&self.web_files, path) {
      let mut response = Response::builder();

      if let Some(content_type) = get_content_type(path) {
        response = response.header(&b"Content-Type"[..], content_type);
      }

      response
        .body(data.into())
        .expect("response.body(Body::from(data))")
    } else {
      Response::builder()
        .status(404)
        .body("".into())
        .expect("Resposne::builder()..body(empty())")
    }
  }
}
