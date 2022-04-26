use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use parse::Type;


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
                if let Err(e) = Server::write(&mut socket, b"200 HELLO.\r\n").await {
                    eprintln!("write error, error info: {}", e);
                    return;
                }
                let text = match Server::read_string(&mut socket).await {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("read string error, error info: {}", e);
                        return;
                    }
                };
                Server::handle(&mut socket, &text).await.unwrap();
                println!("[{}] disconnection connection.", addr);
            });
        }
    }


    async fn handle(s: &mut TcpStream, text: &str) -> io::Result<()> {
        let t = parse::Type::get_type(&text);
        match t {
            Type::Set => {
                println!("set command.");
                Server::write(s, b"0 SUCCESS.").await?
            }
            Type::Get => {}
            Type::Null => Server::write(s, b"1 FAIL.").await?
        }
        Ok(())
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
                Err(e) => break
            }
        }
        let text = unsafe {
            String::from_utf8_unchecked(bufs)
        };
        Ok(text)
    }
}
