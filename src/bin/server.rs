use tokio::net::{TcpListener,TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::fs::File;
use std::io::{Write,BufWriter};
use tokio::time::{sleep, Duration};
use std::net::SocketAddr;

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

async fn process_socket(mut stream: TcpStream,addr:SocketAddr) -> Result<(), Box<dyn Error>>{
    let (rd_stream,wr_stream):(tokio::net::tcp::ReadHalf<'_>, tokio::net::tcp::WriteHalf<'_>) = stream.split();
    rd_stream.readable().await?;
    let mut store:Buffer = Buffer::new(format!("source/async_new_pictures_{}.jpg",addr),8388608);
    let mut buf:Vec<u8> = vec![0u8;8192];
    loop {
        rd_stream.readable().await?;
        match rd_stream.try_read(&mut buf) {
            Ok(0) => {
                println!("{} data read successfully",addr);
                store.flush();
                break
            },
            Ok(size) => {
                //println!("read {} bytes", size);
                if buf[0]==8u8{
                    sleep(Duration::from_secs(3)).await;
                }
                store.write_all(&buf[0..size]);
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("Error WouldBlock");
                continue;
            },
            Err(e) => {
                println!("Error {}",e);
                store.flush();
                return Err(e.into());
            }
        }
    }
    Ok(())
}
 
// cargo run --bin server_async
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener:TcpListener = TcpListener::bind("0.0.0.0:3333").await?;
    /*let std_listener = std::net::TcpListener::bind("0.0.0.0:3333")?;
    std_listener.set_nonblocking(true)?;
    let listener:TcpListener = TcpListener::from_std(std_listener)?;*/

    listener.set_ttl(128).expect("could not set TTL");
    println!("Время жизни пакета {:?}",listener.ttl().unwrap_or(0));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("{:?}",&addr);
         
        tokio::spawn(async move {
           let _ = process_socket(socket,addr).await;
        });
        println!("work in progress, do something!!!");
    }
}