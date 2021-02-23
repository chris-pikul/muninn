use std::net::{TcpStream};
use std::io::{Read,Write};
use std::str::{from_utf8};

fn main() {
    match TcpStream::connect("localhost:1337") {
        Ok(mut stream) => {
            println!("Connected to server {}", stream.peer_addr().unwrap());

            let msg = b"Hello Server";
            stream.write(msg).unwrap();
            println!("Sent message to server");

            let mut buffer = [0 as u8; 16]; //16-byte buffer
            match stream.read(&mut buffer) {
                Ok(size) => {
                    let reply = &buffer[0..size];
                    if reply == msg {
                        println!("Reply is ok!");
                    } else {
                        let reply_text = from_utf8(reply).unwrap();
                        println!("Unexpected reply: {}", reply_text);
                    }
                },
                Err(e) => println!("Error reading reply: {}", e)
            }
        },
        Err(e) => println!("Error: {}", e)
    }
    println!("Terminated");
}
