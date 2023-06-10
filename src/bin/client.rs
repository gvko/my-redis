use std::str::{from_utf8};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    match args.command {
        Command::Get { key } => {
            stream.write_all(b"get ").await?;
            stream.write_all(key.as_bytes()).await?;

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
        }
        Command::Set { key, value } => {
            stream.write_all(b"set ").await?;
            stream.write_all(key.as_bytes()).await?;
            stream.write_all(b" ").await?;
            stream.write_all(value.as_bytes()).await?;

            let mut buf = BytesMut::with_capacity(1024);
            stream.read_buf(&mut buf).await?;

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
        }
    }

    Ok(())
}