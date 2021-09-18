use std::net::TcpListener;
use std::io::{Read};
use std::str;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listenenr = TcpListener::bind(&self.addr).unwrap(); //unwrap handle error if addr already use and kill program

        // loop is a keyword for infinite loop, we can use while but rust include this keyword for that
        // we can add label to our loop, ex : 'outer: loop
        // this allow us in some cases to break for example our 'outer' loop in a inner loop
        // Example:
        // 'outer: loop{
        //    'inner: loop {
        //        break 'outer;
        //    }
        // }
         loop {
             // Pattern matching is better than previous code
             // We can use Ok or Err result from Rust
             match listenenr.accept() {
                 // Here we want only the TCPStream returned by accept method
                 // we don't care for now about Sockaddr
                 Ok((mut stream, _)) => {
                     let mut buf = [0; 1024];
                     match stream.read(&mut buf) {
                         Ok(_) => {
                             println!("Received request: {}", String::from_utf8_lossy(&buf));
                         }
                         Err(e) => {
                             print!("Failed to read from connection {}", e);
                         }
                     }
                 },
                 // We handle error if something wrong
                 Err(e) => {
                     print!("Established connection failure: {}", e);
                 }
             }
         }
    }
}
