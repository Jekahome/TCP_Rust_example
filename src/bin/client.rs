use tokio::net::TcpStream;
use std::error::Error;
use std::time::Duration;
use std::net::SocketAddr;
use tokio::io::BufWriter;

// cargo run --bin client_async 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:3333".parse::<SocketAddr>().unwrap();
    let stream:TcpStream = TcpStream::connect(addr).await?;
    stream.set_nodelay(false).expect("set_nodelay call failed");// включить алгоритм Нэгла, накопить и отправить
    // Теперь, когда вы отключите поток, продолжит ли ОС отправлять данные в фоновом режиме? 
    // Это зависит от задержки и от того, включена она или нет. 
    let _ = stream.set_linger(Some(Duration::from_secs(5)));

    let stream = BufWriter::new(stream);

    let mut payload:Vec<u8> = std::fs::read("source/src.jpg").unwrap();
   
    for (_count,chank) in payload.chunks_mut(8192).enumerate(){
        stream.get_ref().writable().await?;
        match stream.get_ref().try_write(&chank) {
            Ok(_size) => {
                continue;
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("Error WouldBlock");
                continue;
            },
            Err(e) => {
                println!("Error {}",e);
                return Err(e.into());
            }
        };
    }
    Ok(())
}