use bedrock::Server;
use bedrock::request::HttpRequest;
use bedrock::response::HttpResponse;
use std::net::{ Ipv4Addr, IpAddr };
use std::io::Error;

mod request;
mod response;

fn main() {
    let mut server = Server::new(3333, IpAddr::V4(Ipv4Addr::new(127,0 ,0, 1)));
    let _ = server.listen();
    server.register_route("api/v1/hello".to_string(), hello_world);
}

fn hello_world(req: HttpRequest) -> Result<HttpResponse, Error> {
    return Ok(HttpResponse::new("World".to_string()));
}
