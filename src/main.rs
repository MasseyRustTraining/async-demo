use tokio::net::TcpSocket;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

async fn echo(addr: &str) {
    let mut socket = TcpSocket::new_v4()
        .unwrap()
        .connect(addr.parse().unwrap())
        .await
        .unwrap();
    let (mut echo_read, mut echo_write) = socket.split();
    let bytes = b"Hello, world!";
    echo_write.write(bytes).await.unwrap();
    let mut reader = [0u8; 13];
    echo_read.read(&mut reader).await.unwrap();
    assert_eq!(&reader, bytes);
}

#[tokio::main]
async fn main() {
    for _ in 0..3 {
        tokio::spawn(echo("127.0.0.1:7")).await.unwrap();
    }
}
