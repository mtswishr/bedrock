use std::net::{ IpAddr, TcpListener, TcpStream, Ht};
use std::vec::Vec; 
use std::io::{ prelude::*, BufReader, Error };

pub struct Server {
    connections: Vec<Connection>,
    listener: TcpListener
}

struct Request {
    text: String
}

struct Response {
    text: String
}

struct Connection {
    addr: IpAddr
}

struct HttpResponse {}

struct HttpRequest {}

impl Server {
    fn request() -> Response {
        Response { text: "Some response".to_owned() }
    }

    pub fn listen(&self) -> Result<(), Error> {
        for stream in self.listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> Result<(), Error> {
        let buf_reader = BufReader::new(&mut stream);
        let request : Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request: {:#?}", request);
        Ok(())
    }

    fn respond(&self, request: HttpRequest) -> Result<(), Error> {
        Ok(())
    }

    pub fn new(port: i32, addr: IpAddr) -> Server {
        Server{ connections: Vec::new(),
        listener: TcpListener::bind(format!("{}:{}", addr, port)).unwrap()
        }
    }
}
