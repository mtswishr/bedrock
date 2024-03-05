use bedrock::Server;
use std::net::{ Ipv4Addr, IpAddr };

fn main() {
    let server = Server::new(6666, IpAddr::V4(Ipv4Addr::new(127,0 ,0, 1)));
    server.listen();
}
