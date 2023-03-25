use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_quic::QuicSocket;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //simple_logger::SimpleLogger::new().init();

    let mut connection = QuicSocket::bind("0.0.0.0:0")
        .await?
        .connect(Some("google.com"), "172.217.168.206:443")
        .await?;

    let mut stream = connection.incoming().await.unwrap();
    let mut buf: [u8; u16::MAX as usize] = [0; u16::MAX as usize];
    let n = stream.read(&mut buf).await?;
    println!("{}", String::from_utf8_lossy(&buf[..n]));
    stream.shutdown().await?;
    Ok(())
}
