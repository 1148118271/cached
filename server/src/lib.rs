use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    println!("开始启动server.");
    let listener = TcpListener::bind("127.0.0.1:9200").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("收到[{}]的请求.", addr);
        tokio::spawn(async move {
            socket.write_all(b"200 HELLO").await
        });
    }
}