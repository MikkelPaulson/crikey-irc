use std::io;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

mod connection;
mod dispatcher;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6667").expect("Could not connect to server.");

    let mut connection = connection::Connection::new(&stream);
    connection.send_command(connection::Command::Nick {
        nickname: String::from("foo"),
        hopcount: None,
    })?;
    connection.send_command(connection::Command::User {
        username: String::from("pjohnson"),
        hostname: String::from("local"),
        servername: String::from("remote"),
        realname: String::from("Potato Johnson"),
    })?;

    // hangs on close!
    loop {
        if let Some(data) = connection.poll() {
            continue;
        }
        thread::sleep(Duration::from_millis(100));
    }

    //Ok(())
}
