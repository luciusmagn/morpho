#![allow(trivial_casts)]

use conduit_mime_types as mime;
use conduit::{
	box_error, header, Body, Handler, HandlerResult, RequestExt, Response, StatusCode,
};
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct Static {
	path: PathBuf,
	types: mime::Types,
}

impl Static {
	pub fn new<P: AsRef<Path>>(path: P) -> Static {
		Static {
			path: path.as_ref().to_path_buf(),
			types: mime::Types::new().expect("Couldn't load mime-types"),
		}
	}
}

impl Handler for Static {
	#[allow(deprecated)]
	fn call(&self, request: &mut dyn RequestExt) -> HandlerResult {
		let request_path = &request.path()[1..];
		if request_path.contains("..") {
			return Ok(not_found());
		}

		let path = self.path.join(request_path);
		let mut mime = self.types.mime_for_path(&path);
		let file = match File::open(&path) {
			Ok(f) => {
				if path.is_dir() {
					mime = self.types.mime_for_path(&path.join("index.html"));
					match File::open(&path.join("index.html")) {
						Ok(f) => f,
						Err(..) => return Ok(not_found()),
					}
				} else {
					f
				}
			}
			Err(..) => return Ok(not_found()),
		};
		let data = file.metadata().map_err(box_error)?;
		if data.is_dir() {
			return Ok(not_found());
		}

		Response::builder()
			.header(header::CONTENT_TYPE, mime)
			.header(header::CONTENT_LENGTH, data.len())
			.body(Body::File(file))
			.map_err(box_error)
	}
}

fn not_found() -> Response<Body> {
	Response::builder()
		.status(StatusCode::NOT_FOUND)
		.header(header::CONTENT_LENGTH, 0)
		.header(header::CONTENT_TYPE, "text/plain")
		.body(Body::empty())
		.unwrap()
}
