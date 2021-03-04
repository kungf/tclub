//"The world’s programmers are one family" -- wyang
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let whoami = "127.0.0.1:65511"
    let listener = TcpListener::bind(whoami)?;

    let mut stream = TcpStream::connect("127.0.0.1:65533")?;
    stream.write(whoami).unwrap();

    let stdin = io::stdin();
    loop {
        println!("[you]>");
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input == "q!\n" {
            break;
        }

        stream.write(input.as_bytes()).unwrap();

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer).unwrap();
        println!("[stranger]>{}", str::from_utf8(&buffer).unwrap());
    }
    Ok(())
}
