#[derive(Debug)]
pub struct HttpRequest {
    pub request: String,
}

impl HttpRequest {
    pub fn new(req: String) -> HttpRequest {
        HttpRequest{ request: req }
    }
}
