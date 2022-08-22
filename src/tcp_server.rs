use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::{str, thread};

pub fn serve(address: &str) -> Result<(), failure::Error>{
    let listener = TcpListener::bind(address)?;
    loop{
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<(), failure::Error>{
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop{
        let nbyte = stream.read(&mut buffer)?;
        if nbyte == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbyte])?);
        stream.write_all(&buffer[..nbyte])?;
    }
}