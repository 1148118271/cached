use tokio::io;
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
        let conf = config::default();
        let url = format!("0.0.0.0:{}", conf.get_port());
        log::info!("run server [{}]", url);
        let listener = TcpListener::bind(url).await?;
        Ok(Server(listener))
    }

    pub async fn run(&self) -> io::Result<()> {
        println!("server started.");
        loop {
            let (socket, addr) = self.0.accept().await?;
            println!("[{}] client connection.", addr);
            tokio::spawn(async move {
                let mut framed = BytesCodec::new().framed(socket);
                let stream = framed.get_ref();
                if let Err(e) = Server::write(&stream, b"200 HELLO").await {
                    eprintln!("write error, error info: {}", e);
                    return;
                }
                while let Some(message) = framed.next().await {
                    match message {
                        Ok(bytes) => {
                            let text = unsafe {
                                String::from_utf8_unchecked(bytes.to_vec())
                            };
                            println!("{:?}", bytes);
                            let stream = framed.get_ref();
                            if let Err(e) = Server::handle(stream, &text).await {
                                eprintln!("handle data error, error info: {}", e);
                                return;
                            }
                        }
                        Err(err) => {
                            println!("socket closed with error: {:?}", err);
                            return;
                        }
                    }
                }
                println!("socket closed")
            });
            println!("线程结束");
        }
    }


    async fn handle(s: &TcpStream, text: &str) -> io::Result<()> {
        let t = Type::get_type(&text);
        match t {
            Type::Set => {
                if let Ok(v) = SetParse::new(text) {
                    buffer::set(v.key, v.value);
                    Server::success(s).await?;
                    return Ok(());
                }
                Server::fail(s).await?;
            }
            Type::Get => {
                if let Ok(v) = GetParse::new(text) {
                    match buffer::get(v.key) {
                        None => Server::write(s, b"0").await?,
                        Some(val) => Server::write(s, format!("0 {}", val).as_bytes()).await?
                    }
                    return Ok(());
                }
                Server::fail(s).await?;
            }
            Type::Null => Server::fail(s).await?
        }
        Ok(())
    }

    async fn success(s: &TcpStream) -> io::Result<()> {
        Server::write(s, b"0 SUCCESS.").await
    }

    async fn fail(s: &TcpStream) -> io::Result<()> {
        Server::write(s, b"1 FAIL.").await
    }

    async fn write(s: &TcpStream, buf: &[u8]) -> io::Result<()> {
        s.writable().await?;
        s.try_write(buf)?;
        Ok(())
    }

}
