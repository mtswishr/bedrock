#[derive(Debug)]
pub struct HttpResponse {
    response: String,
}

impl HttpResponse {
    pub fn new(res: String) -> HttpResponse {
        HttpResponse{ response: res }
    }
}
