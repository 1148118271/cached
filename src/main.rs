use std::io::Read;
use std::net::TcpStream;

#[tokio::main]
async fn main() {
    let result = server::server().await;
}

#[test]
fn c() {
    let mut stream = TcpStream::connect("127.0.0.1:9200").unwrap();
    let mut v = String::new();
    stream.read_to_string(&mut v);
    println!("{:?}", v)
}