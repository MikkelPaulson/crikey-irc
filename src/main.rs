use std::io;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

mod connection;
mod dispatcher;
mod terminal;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6667").expect("Could not connect to server.");

    let dispatcher = dispatcher::Dispatcher::new();

    let mut connection = connection::Connection::new(&stream, dispatcher);

    let terminal = terminal::Terminal::new(io::stdin());

    connection.send_command(connection::Command::Nick {
        nickname: "foo".to_string(),
        hopcount: None,
    })?;
    connection.send_command(connection::Command::User {
        username: "pjohnson".to_string(),
        hostname: "local".to_string(),
        servername: "remote".to_string(),
        realname: "Potato Johnson".to_string(),
    })?;

    loop {
        if connection.poll() {
            continue;
        }

        if let Some(mut input) = terminal.read() {
            input.pop(); // trim trailing newline
            connection.send_command_raw(input)?;
        }

        thread::sleep(Duration::from_millis(100));
    }

    //Ok(())
}
