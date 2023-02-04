use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            println!("{}", String::from_utf8_lossy(&data[0..size]));

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

/*fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            println!("{:?}", &data[0..size]);
            // write the data back to the stream
            match stream.write(&data[0..size]) {
                Ok(_) => {
                    println!("Data sent successfully to {}", stream.peer_addr().unwrap());
                },
                Err(e) => {
                    println!("An error occurred while sending data to {}: {}", stream.peer_addr().unwrap(), e);
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }
        },
        Err(e) => {
            println!("An error occurred while reading data from {}: {}", stream.peer_addr().unwrap(), e);
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}*/
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}