use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::time::{Duration, Instant};
use std::io::{BufWriter};

// cargo run --bin client 
fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            stream.set_write_timeout(Some(Duration::new(0, 100))).expect("set_write_timeout call failed");
            stream.set_nodelay(false).expect("set_nodelay call failed");// включить алгоритм Нэгла, накопить и отправить
             
            let mut stream = BufWriter::new(stream);

            let now = Instant::now();
            let mut payload:Vec<u8> = std::fs::read("source/9.jpg").unwrap();
            for (count,chank) in payload.chunks_mut(4096).enumerate(){
                stream.write(&chank).unwrap(); 
                std::thread::sleep(Duration::new(0, 100));// Это важно, способ притормозить канал
            }
             

            std::thread::sleep(Duration::new(0, 500));
            stream.flush().unwrap();
            stream.get_mut().shutdown(std::net::Shutdown::Write); 
            
            println!("Time:{} sec", now.elapsed().as_secs());// 4 sec 4 Gb
             
            let msg = b"1";
            let mut data = [0 as u8; 1]; 
            match stream.get_mut().read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}