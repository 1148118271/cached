use std::io::Write;
use bytes::buf::Writer;
use bytes::{BufMut, BytesMut};
use tokio::io;
use tokio::io::{AsyncWriteExt, Interest};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Decoder};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use parse::{Parse, Type};
use parse::get::GetParse;
use parse::set::SetParse;



pub struct Server(TcpListener);


impl Server {
    pub async fn new() -> io::Result<Server> {
        let listener = TcpListener::bind("0.0.0.0:9200").await?;
        Ok(Server(listener))
    }

    pub async fn run(&self) -> io::Result<()> {
        println!("server started.");
        loop {
            let (mut socket, addr) = self.0.accept().await?;
            println!("[{}] client connection.", addr);
            tokio::spawn(async move {

                let mut framed = BytesCodec::new().framed(socket);
                let x = framed.get_mut();
                if let Err(e) = Server::write(x, b"200 HELLO").await {
                    eprintln!("write error, error info: {}", e);
                    return;
                }
                while let Some(message) = framed.next().await {
                    match message {
                        Ok(mut bytes) => {
                            // let text = unsafe {
                            //     String::from_utf8_unchecked(bytes.to_vec())
                            // };
                            println!("{:?}", bytes);
                            x.write_all(b"ssss").await.unwrap();
                            x.flush().await.unwrap();
                            println!("writer")
                            // let mut w: Writer<BytesMut> = bytes.writer();
                            // Server::handle(&mut w, &text)
                        }
                        Err(err) => println!("Socket closed with error: {:?}", err),
                    }
                }
                println!("Socket received FIN packet and closed connection");
            });
        }
    }


    async fn handle(s: &mut TcpStream, text: &str) -> io::Result<()> {
        let t = Type::get_type(&text);
        match t {
            Type::Set => {
                if let Ok(v) = SetParse::new(text) {
                    buffer::set(v.key, v.value);
                    Server::success(s).await?;
                }
                Server::fail(s).await?;
            }
            Type::Get => {}
            Type::Null => Server::fail(s).await?
        }
        Ok(())
    }

    async fn success(s: &mut TcpStream) -> io::Result<()> {
        Server::write(s, b"0 SUCCESS.").await
    }

    async fn fail(s: &mut TcpStream) -> io::Result<()> {
        Server::write(s, b"1 FAIL.").await
    }

    async fn write(s: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
        s.writable().await?;
        s.try_write(buf)?;
        Ok(())
    }

    async fn read_string(s: &mut TcpStream) -> io::Result<String> {
        let mut buf = [0; 1024];
        let mut bufs = vec![];
        loop {
            s.readable().await?;
            match s.try_read(&mut buf) {
                Ok(n) =>  bufs.extend_from_slice(&buf[..n]),
                Err(e) => {
                    println!("e > {}", e);
                    break
                }
            }
        }
        let text = unsafe {
            String::from_utf8_unchecked(bufs)
        };
        Ok(text)
    }
}
