use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 64]; //64-byte buffer
    while match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Received message from client, echoing back");

            //Correctly read from the stream
            //ECHO back
            stream.write(&buffer[0..size]).unwrap();

            false
        },
        Err(_) => {
            //Something went wrong
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();

            false
        }
    }{}
}

fn main() {
    //Create a new listener
    let listener = TcpListener::bind("0.0.0.0:1337").expect("Unable to bind to address 0.0.0.0:1337");
    println!("Listening on port 1337");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    //Handle the client
                    handle_client(stream);
                    println!("Leaving connection thread");
                });
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    //Drop the socket on closing
    drop(listener);
}