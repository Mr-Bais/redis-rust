use std::net::TcpListener;
use std::io::{Read, Write};

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(size) => stream.write_all(&buffer[..size]).unwrap(),
                        Err(e) => {
                            println!("failed to read {e}");
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("failed to connect {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    use std::sync::Once;

    static INIT: Once = Once::new();
    fn setup_server() {
        INIT.call_once(|| {
            thread::spawn(|| {
                main();
            });
            thread::sleep(Duration::from_millis(100));
        });
    }

    #[test]
    fn test_server_listens() {
        //Start the server in a separate background thread
        setup_server();

        let connection = TcpStream::connect("127.0.0.1:6379");
        assert!(connection.is_ok(), "server is not running on port 6379");
    }

    #[test]
    fn test_client_echo() {
        setup_server();

        let mut connection = TcpStream::connect("127.0.0.1:6379").unwrap();

        let message = b"hello there";
        connection.write_all(message).unwrap();

        let mut buffer = [0; 512];
        let byte_read = connection.read(&mut buffer).unwrap();

        assert_eq!(&buffer[..byte_read], message);
    }
}

