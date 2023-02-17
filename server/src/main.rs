use std::io::{stderr, stdout, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
//use std::fs::File;
//use std::io::BufWriter;
//use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    /*let file_name = "out-s16-audio.raw";
    let mut file = File::create(file_name).unwrap();*/
    let mut data = vec![0; 480];
    while match stream.read(unsafe { std::mem::transmute::<&mut [i16], &mut [u8]>(&mut data) }) {
        Ok(size) => {
            //file.write_all(unsafe { std::mem::transmute::<&[i16], &[u8]>(&data[0..size / 2]) }).unwrap();
            //println!("Data written successfully to {}", file_name);

            // write the data back to the stream
            match stream.write(unsafe { std::mem::transmute::<&[i16], &[u8]>(&data) }) {
                Ok(_) => {
                    //println!("Data sent successfully to {}", stream.peer_addr().unwrap());
                }
                Err(e) => {
                    println!(
                        "An error occurred while sending data to {}: {}\n",
                        stream.peer_addr().unwrap(),
                        e
                    );
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                }
            }
            true
        }
        Err(e) => {
            println!(
                "An error occurred while reading data from {}: {}\n",
                stream.peer_addr().unwrap(),
                e
            );
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}
fn find_first_available_port(start_port: u16) -> Option<u16> {
    let mut port = start_port;
    loop {
        match TcpListener::bind(("0.0.0.0", port)) {
            Ok(listener) => {
                // Port is available
                drop(listener);
                return Some(port);
            }
            Err(_) => {
                // Port is in use, try next port
                port += 1;
                if port > u16::MAX {
                    // Reached the maximum port number without finding an available port
                    return None;
                }
            }
        }
    }
}
fn main() {
    let port = find_first_available_port(2224).unwrap();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    // accept connections and process them, spawning a new thread for each one
    let stderr = stderr();
    stderr.lock().write_all(format!("Server listening on port {}\n", port).as_bytes()).unwrap();
    stderr.lock().flush().unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stderr.lock().write_all(format!("New connection: {}\n", stream.peer_addr().unwrap()).as_bytes()).unwrap();
                stderr.lock().flush().unwrap();
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                stderr.lock().write_all(format!("Error: {}\n", e).as_bytes()).unwrap();
                stderr.lock().flush().unwrap();
                // connection failed
            }
        }
    }
    // close the socket server
    drop(listener);
}
