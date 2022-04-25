use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{ReadHalf, WriteHalf};
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
            let (mut r, mut w) = socket.into_split();
            println!("[{}] client connection.", addr);
            tokio::spawn(async move {

                w.write(b"200 HELLO.\r\n").await.unwrap();
                // let mut buf = [0; 1024];
                // let i = r.read(&mut buf).await.unwrap();
                // let text = unsafe {
                //     String::from_utf8_unchecked((&buf[..i]).to_vec())
                // };
                // println!("{}", text);
                // let t = parse::Type::get_type(&text);
                // match t {
                //     Type::Set => {
                //         println!("set command.");
                //         w.write(b"1 success.\r\n").await.unwrap();
                //     }
                //     Type::Get => {},
                //     Type::Null => {}
                // }
                // w.write(b"200 HELLO.\r\n").await.unwrap();
                // if let Err(e) = Server::write(&mut w, b"200 HELLO.\r\n").await {
                //     eprintln!("write error, error info: {}", e);
                //     return;
                // }
                // let text = match Server::read_string(&mut r).await {
                //     Ok(text) => text,
                //     Err(e) => {
                //         eprintln!("read string error, error info: {}", e);
                //         return;
                //     }
                // };
                // w.write(b"1 success.\r\n").await.unwrap();
                // Server::handle(&mut w, &text).await.unwrap();
                // w.write(b"1 success.\r\n").await.unwrap();
                // println!("[{}] disconnection connection.", addr);
            });
        }
    }


    async fn handle(socket: &mut WriteHalf<'_>, text: &str) -> Result<(), ()> {
        let t = parse::Type::get_type(&text);
        match t {
            Type::Set => {
                println!("set command.");
            }
            Type::Get => {}
            Type::Null => {return Err(())}
        }
        Ok(())
    }

    async fn write(w: &mut WriteHalf<'_>, buf: &[u8]) -> io::Result<()> {
        loop {
            match w.write(buf).await {
                Ok(_) => break,
                Err(e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    async fn read_string(r: &mut ReadHalf<'_>) -> io::Result<String> {
        // socket.readable().await?;
        let mut buf = [0; 1024];
        let mut bufs = vec![];
        loop {
            match r.read(&mut buf).await {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    bufs.extend_from_slice(&buf[..n]);
                    continue;
                },
                Err(e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        let text = unsafe {
            String::from_utf8_unchecked(bufs)
        };
        Ok(text)
    }
}
