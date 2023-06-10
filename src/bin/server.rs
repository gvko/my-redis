use std::io::Error;
use tokio::net::{TcpListener, TcpStream};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use my_redis::*;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let address = "127.0.0.1:8081".to_string();
    let listener = TcpListener::bind(&address).await?;
    println!("\n---\nServer started on {address}. Waiting for connections...");

    let mut db = Db::new();
    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut buf = BytesMut::with_capacity(1025);
        socket.read_buf(&mut buf).await?;

        let command_words = buffer_to_array(&mut buf);
        let command = Command::get_command(&command_words[0]);

        println!("buffer: {:#?}", buf);
        process_command(command, command_words, &mut socket, &mut db).await?;
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn process_command(command: Command, command_words: Vec<String>, socket: &mut TcpStream, db: &mut Db) -> Result<(), Error> {
    match command {
        Command::Get => {
            Ok(())
        }
        Command::Set => {
            let response = db.write(&command_words);

            match response {
                Ok(result) => {
                    println!("SET response: {:?}", result);
                    socket.write_all(&result.as_bytes()).await?;
                }
                Err(err) => {
                    eprintln!("SET error: {:?}", err);
                    socket.write_all(&err.as_bytes()).await?;
                }
            }

            Ok(())
        }
        Command::Invalid => {
            eprintln!("Invalid command: {:?}", command_words);
            Ok(())
        }
    }
}