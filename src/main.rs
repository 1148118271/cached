use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::Add;

#[tokio::main]
async fn main() {
    let server = server::Server::new().await.unwrap();
    server.run().await.unwrap();
}

#[test]
fn c() {
    let mut stream = TcpStream::connect("127.0.0.1:9200").unwrap();
    let mut s = [0; 128];
    let u = stream.read(&mut s).unwrap();
    println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
    let mut string = String::new();
    for i in 0..101600 {
        string = string.add("s");
    }
    let string1 = format!("set key {}\r\n", string);
    stream.write_all(string1.as_bytes());
    stream.flush();
    let u = stream.read(&mut s).unwrap();
    println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
}