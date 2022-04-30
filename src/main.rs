
#[tokio::main]
async fn main() {
    local_log::enable_logging();
    let server = server::Server::new().await.unwrap();
    server.run().await.unwrap();
}


// use std::io::{Read, Write};
// use std::net::{Shutdown, TcpStream};
//
// #[test]
// fn c() {
//     let mut stream = TcpStream::connect("127.0.0.1:9200").unwrap();
//     let mut s = [0; 128];
//     let u = stream.read(&mut s).unwrap();
//     println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
//     stream.write_all(b"set test gaoxiangkang");
//     stream.flush();
//     let u = stream.read(&mut s).unwrap();
//     println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
//     stream.write_all(b"get test").unwrap();
//     stream.flush();
//     let u = stream.read(&mut s).unwrap();
//     println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
//
//     stream.write_all(b"rm test").unwrap();
//     stream.flush();
//     let u = stream.read(&mut s).unwrap();
//     println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
//
//
//     stream.write_all(b"get test").unwrap();
//     stream.flush();
//     let u = stream.read(&mut s).unwrap();
//     println!("{:?}", String::from_utf8((&s[..u]).to_vec()).unwrap());
//
//     stream.shutdown(Shutdown::Both).expect("shutdown call failed");
// }