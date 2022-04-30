use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Decoder};
use tokio_stream::StreamExt;
use parse::{Parse, Type};
use parse::get::GetParse;
use parse::remove::RmParse;
use parse::set::SetParse;



pub struct Server(TcpListener);


impl Server {
    pub async fn new() -> io::Result<Server> {
        let conf = config::default();
        let url = format!("0.0.0.0:{}", conf.get_port());
        log::info!("[{}] server run.", url);
        let listener = TcpListener::bind(url).await?;
        Ok(Server(listener))
    }

    pub async fn run(&self) -> io::Result<()> {
        loop {
            let (socket, addr) = self.0.accept().await?;
            log::info!("[{}] client connection.", addr);
            tokio::spawn(async move {
                let mut framed = BytesCodec::new().framed(socket);
                let stream = framed.get_ref();
                if let Err(e) = Server::write(&stream, b"200").await {
                    log::error!("write error, error info: {}", e);
                    return;
                }
                while let Some(message) = framed.next().await {
                    match message {
                        Ok(bytes) => {
                            let text = unsafe {
                                String::from_utf8_unchecked(bytes.to_vec())
                            };
                            log::debug!("client => server [{:?}]", bytes);
                            let stream = framed.get_ref();
                            if let Err(e) = Server::handle(stream, &text).await {
                                log::error!("handle data error, error info: {}", e);
                                return;
                            }
                        }
                        Err(_) => {
                            return;
                        }
                    }
                }
                log::info!("[{}] client closed.", addr);
            });
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
            Type::Rm => {
                if let Ok(v) = RmParse::new(text) {
                    match buffer::remove(v.key) {
                        None => Server::write(s, b"0").await?,
                        Some(val) => Server::write(s, format!("0 {}", val).as_bytes()).await?
                    }
                    return Ok(());
                }
                Server::fail(s).await?;
            }
            Type::Null => Server::fail(s).await?,
        }
        Ok(())
    }

    async fn success(s: &TcpStream) -> io::Result<()> {
        Server::write(s, b"0").await
    }

    async fn fail(s: &TcpStream) -> io::Result<()> {
        Server::write(s, b"1").await
    }

    async fn write(s: &TcpStream, buf: &[u8]) -> io::Result<()> {
        log::debug!("server => client [b\"{}\"]", String::from_utf8(buf.to_vec()).expect("result info parse exception."));
        s.writable().await?;
        s.try_write(buf)?;
        Ok(())
    }

}
