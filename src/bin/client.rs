use std::str::{from_utf8};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    stream.write_all(b"set foo bar").await?;
    let mut buf = BytesMut::with_capacity(1024);
    let _stream_length = stream.read_buf(&mut buf).await?;

    match from_utf8(&mut buf) {
        Ok(res) => {
            if res == "r Ok" {
                println!("key updated");
            } else if res == "Ok" {
                println!("key set");
            } else {
                eprintln!("Unrecognized response: {}", res);
            }
        }
        Err(err) => eprintln!("Error: {}", err)
    }

    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    stream.write_all(b"get foo").await?;
    let mut buf = BytesMut::with_capacity(1024);
    stream.read_buf(&mut buf).await?;

    match from_utf8(&mut buf) {
        Ok(res) => {
            if res == "" {
                println!("No key found");
            } else {
                println!("value: {}", res);
            }
        }
        Err(err) => eprintln!("Error: {}", err)
    }

    Ok(())
}