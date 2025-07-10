

## [std::net](https://doc.rust-lang.org/std/net/index.html)

 > TCP - это протокол нижнего уровня, который описывает детали того, как информация поступает с одного сервера на другой, но не указывает, что это за информация. 
 > HTTP строит поверх TCP, определяя содержание запросов и ответов.
 
### Для передачи по TCP в качестве полезной нагрузки используется картинка src.jpg 

### TCP server/client

```
 $ cargo run --bin server
 $ cargo run --bin client
```

### Async TCP server/client

```
 $ cargo run --bin server_async
 $ cargo run --bin client_async
```
 
## Links
 
[std::io::BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html)

[std::io::BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)

[async Tcp Tokio](https://tokio.rs/tokio/tutorial)

[tokio::net::TcpStream](https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html)

