use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
//use std::fs::OpenOptions;
use std::io::BufWriter;
use std::fs::File;
use std::time::{Duration, Instant};

struct Buffer(Option<BufWriter<std::fs::File>>);
impl Buffer {
    fn new<P: AsRef<std::path::Path>>(&mut self,file: P,capacity:usize){
        if self.0.is_none(){
           self.0 = Some( BufWriter::with_capacity(capacity, File::create(file).unwrap()));  
        } 
    }
    fn write_all(&mut self,d:&[u8]) {
        if let Some(ref mut buff) = self.0{
            buff.write(d);
        }
    }
    fn flush(&mut self){
        if let Some(ref mut buff) = self.0{
            buff.flush().unwrap();
            self.0 = None;
        }
    }
}

static mut BUFFER:Buffer = Buffer(None);// способ использовать один буффер для всех частей потока
static mut DATA:[u8;4096] = [0_u8; 4096];// способ убрать выделение памяти для масссива для каждого запроса

fn handle_connection(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::new(0, 50))).expect("set_read_timeout call failed");
    //println!("stream read timeout {:?}",stream.read_timeout().unwrap());// 4ms
    unsafe{ 
 
    unsafe{BUFFER.new("source/iteration.jpg",8388608);}//8388608 8Mb, 16777216 16Mb
    
    while match stream.read(&mut DATA) {
        Ok(size) => {

             if size > 0 {
                unsafe{BUFFER.write_all(&DATA[0..size]);}
             }
            
            // Канал захлёбываться при отсутствии синхронизации с писателем
            // Error:Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }

            if let Err(e) = stream.write(b"1"){
                eprintln!("Error:{:?}",e);
            }
            if size > 0{  
                true 
            }else{
                unsafe{ BUFFER.flush();}
                stream.shutdown(Shutdown::Both);
                false
            }
           
        },
        Err(e) => {
           
            unsafe{ BUFFER.flush();}

            println!("Err:{:?}",e);
          
            if let Ok(addr) = stream.peer_addr(){
                println!("Произошла ошибка, соединение с {}", addr);
            }
            stream.shutdown(Shutdown::Both);
            
            false
        }
    } {}
}
}

// cargo run --bin server 
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    listener.set_ttl(100).expect("could not set TTL");
    println!("Время жизни пакета {:?}",listener.ttl().unwrap_or(0));// default 64

    unsafe{BUFFER.new("source/iteration.jpg",8388608);}

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
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}