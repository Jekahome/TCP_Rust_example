#![allow(unused_unsafe)]

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, BufWriter};
use std::fs::File;
use std::time::Duration;

pub struct Buffer(BufWriter<std::fs::File>);
impl Buffer {
    pub fn new<P: AsRef<std::path::Path>>(file: P, capacity: usize) -> Self{
        Buffer(BufWriter::with_capacity(capacity, File::create(file).unwrap()))  
    }
    pub fn write_all(&mut self,d:&[u8]) {
        let _ = self.0.write(d);
    }
    pub fn flush(&mut self){
        self.0.flush().unwrap(); 
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::new(0, 50))).expect("set_read_timeout call failed");
    //println!("stream read timeout {:?}",stream.read_timeout().unwrap());// 4ms
  
    let mut store:Buffer = Buffer::new("source/new_pictures.jpg",8388608);//8388608 8Mb, 16777216 16Mb
    let mut buf:Vec<u8> = vec![0u8;8192];

    while match stream.read(&mut buf) {
        Ok(size) => {
             if size > 0 {
                println!("read {} bytes", size);
                store.write_all(&buf[0..size]);
             }
            
            // Канал захлёбываться при отсутствии синхронизации с писателем
            // Error:Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }

            if let Err(e) = stream.write(b"1"){
                eprintln!("Error:{:?}",e);
            }
            if size > 0{  
                true 
            }else{
                store.flush();
                let _ = stream.shutdown(Shutdown::Both);
                false
            }
        },
        Err(e) => {
           
            store.flush();

            println!("Err:{:?}",e);
          
            if let Ok(addr) = stream.peer_addr(){
                println!("Произошла ошибка, соединение с {}", addr);
            }
            let _ = stream.shutdown(Shutdown::Both);
            
            false
        }
    } {}

}

// cargo run --bin server 
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    listener.set_ttl(100).expect("could not set TTL");
    println!("Время жизни пакета {:?}",listener.ttl().unwrap_or(0));// default 64

    // итератор по соединениям 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection client: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_connection(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed 
            }
        }
    }
    // close the socket server
    drop(listener);
}