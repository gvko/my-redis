use tokio::net::TcpListener;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let address = "127.0.0.1:8081".to_string();
    let listener = TcpListener::bind(&address).await?;

    println!("\n---\nServer started on {address}. Waiting for connections...");
    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut buf = BytesMut::with_capacity(1025);
        socket.read_buf(&mut buf).await?;

        println!("buffer: {:#?}", buf);
    }

    #[allow(unreachable_code)]
    Ok(())
}