//use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

mod connection;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6667").expect("Could not connect to server.");

    stream.write(b"NICK foo\r\n")?;
    stream.write(b"USER pjohnson local remote :Potato Johnson\r\n")?;

    let mut connection = connection::Connection::new(&stream);
    //connection.send_command_raw("NICK foo\r\n".to_string())?;
    //connection.send_command_raw("USER pjohnson local remote :Potato Johnson\r\n".to_string())?;

    // hangs on close!
    loop {
        if let Some(data) = connection.poll() {
            print!("{}", data);
            continue;
        }
        thread::sleep(Duration::from_millis(100));
    }

    //Ok(())
}
