use std::net::{ IpAddr, TcpListener, TcpStream };
use std::vec::Vec; 
use std::io::{ prelude::*, BufReader, Error };
use std::collections::HashMap;

pub mod request;
pub mod response;

use crate::request::HttpRequest;
use crate::response::HttpResponse;

pub struct Server {
    connections: Vec<Connection>,
    listener: TcpListener,
    routes: HashMap<String, fn(HttpRequest) -> Result<HttpResponse, Error>>,
} 

struct Connection {
    addr: IpAddr
}


impl Server {
    pub fn new(port: i32, addr: IpAddr) -> Server {
        Server{ connections: Vec::new(),
        listener: TcpListener::bind(format!("{}:{}", addr, port)).unwrap(),
        routes: HashMap::new()
        }
    }

    pub fn listen(&self) -> Result<(), Error> {
        for stream in self.listener.incoming() {
            let _ = self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> Result<&str, Error> {
        let buf_reader = BufReader::new(&mut stream);
        let request: HttpRequest  = HttpRequest::new(buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect());

        self.routes.get(&request.request);
        Ok("Hello")
    }

    pub fn register_route(&mut self, route: String, callback: fn(HttpRequest) -> Result<HttpResponse, Error>) {
        self.routes.insert(route, callback);
    }
}
