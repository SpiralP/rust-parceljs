use super::{get_content_type, get_file};
use warp::{http::Response, path::FullPath, reply, reply::Reply};

pub struct ParceljsResponder {
  path: FullPath,
}

impl ParceljsResponder {
  pub fn new(path: FullPath) -> Self {
    Self { path }
  }
}

impl Reply for ParceljsResponder {
  fn into_response(self) -> reply::Response {
    let path = self.path.as_str();

    if let Ok(data) = get_file(path) {
      let mut response = Response::builder();

      if let Some(content_type) = get_content_type(path) {
        response.header(&b"Content-Type"[..], content_type);
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
