use crate::connection::Connect;
use std::cell::RefCell;
use std::io;
use std::net;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

mod client;
mod connection;
mod terminal;

pub fn run<A: net::ToSocketAddrs>(addr: A) -> io::Result<()> {
    let stream = net::TcpStream::connect(addr).expect("Could not connect to server.");

    let connection = Rc::new(RefCell::new(connection::Connection::new(stream)));

    let client = client::Client::new(connection.clone());

    let terminal = terminal::Terminal::new(io::stdin());

    connection
        .borrow_mut()
        .send_command(connection::Command::Nick {
            nickname: "baz".to_string(),
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
