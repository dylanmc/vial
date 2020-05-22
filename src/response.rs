use {
    crate::{asset, util},
    std::{
        fmt, fs,
        io::{self, Read},
        path::Path,
    },
};

pub struct Response {
    pub code: usize,
    pub body: String,
    pub content_type: String,
    pub reader: Option<Box<dyn Read>>,
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("code", &self.code)
            .field("content_type", &self.content_type)
            .field("body", &self.body)
            .field("reader", &self.reader.is_some())
            .finish()
    }
}

impl Default for Response {
    fn default() -> Response {
        Response {
            code: 200,
            body: String::new(),
            content_type: "text/html; charset=utf8".to_string(),
            reader: None,
        }
    }
}

impl Response {
    pub fn new() -> Response {
        Response::default()
    }

    pub fn from<T: Into<Response>>(from: T) -> Response {
        from.into()
    }

    pub fn from_file(path: &str) -> Response {
        Response::default().with_file(path)
    }

    pub fn with_code(mut self, code: usize) -> Response {
        self.code = code;
        self
    }

    pub fn with_body(mut self, body: &str) -> Response {
        self.body = body.to_string();
        self
    }

    pub fn with_file(mut self, path: &str) -> Response {
        match fs::read_to_string(asset::normalize_path(path)) {
            Ok(body) => {
                println!("CT: {}", util::content_type(path));
                self.content_type = util::content_type(path).to_string();
                self.body = body;
            }

            Err(e) => {
                self.body = format!("<h1>500 Internal Error</h1><pre>{:?}", e);
                self.code = 500;
            }
        }
        self
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.reader.is_some() {
            write!(f, "(body is io::Read)")
        } else {
            write!(f, "{}", self.body)
        }
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Response {
        Response {
            body: s.to_string(),
            ..Response::default()
        }
    }
}

impl From<&String> for Response {
    fn from(s: &String) -> Response {
        Response {
            body: s.clone(),
            ..Response::default()
        }
    }
}

impl From<String> for Response {
    fn from(body: String) -> Response {
        Response {
            body,
            ..Response::default()
        }
    }
}

impl From<usize> for Response {
    fn from(i: usize) -> Response {
        Response {
            code: i.into(),
            ..Response::default()
        }
    }
}

impl From<std::borrow::Cow<'_, [u8]>> for Response {
    fn from(i: std::borrow::Cow<'_, [u8]>) -> Response {
        Response {
            body: String::from_utf8_lossy(&i).to_string(),
            ..Response::default()
        }
    }
}
