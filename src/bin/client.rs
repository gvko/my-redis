use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    stream.write_all(b"set foo bar").await?;

    Ok(())
}