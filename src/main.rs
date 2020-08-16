use crate::connection::Connect;
use std::cell::RefCell;
use std::io;
use std::net::TcpStream;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

mod client;
mod connection;
mod dispatcher;
mod terminal;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6667").expect("Could not connect to server.");

    let dispatcher = Rc::new(RefCell::new(dispatcher::Dispatcher::new()));

    let connection = Rc::new(RefCell::new(connection::Connection::new(
        &stream,
        dispatcher.clone(),
    )));

    let client = client::Client::new(
        connection.clone(),
        dispatcher.clone()
    );

    let terminal = terminal::Terminal::new(io::stdin());

    connection
        .borrow_mut()
        .send_command(connection::Command::Nick {
            nickname: "foo".to_string(),
            hopcount: None,
        })?;
    connection
        .borrow_mut()
        .send_command(connection::Command::User {
            username: "pjohnson".to_string(),
            hostname: "local".to_string(),
            servername: "remote".to_string(),
            realname: "Potato Johnson".to_string(),
        })?;

    loop {
        if connection.borrow_mut().poll() {
            continue;
        }

        if let Some(mut input) = terminal.read() {
            input.pop(); // trim trailing newline
            connection.borrow_mut().send_command_raw(input)?;
        }

        thread::sleep(Duration::from_millis(100));
    }

    //Ok(())
}
