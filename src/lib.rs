use std::io;
use std::net;
use std::thread;
use std::time::Duration;

mod client;
mod connection;
mod terminal;

pub fn run<A: net::ToSocketAddrs>(
    addr: A,
    nickname: String,
    username: String,
    realname: String,
) -> io::Result<()> {
    let token = client::AuthToken {
        nickname: nickname.parse().unwrap(),
        username: username.parse().unwrap(),
        mode: 0,
        realname: realname,
        password: None,
    };

    let mut client = client::Client::connect(addr, token);

    let terminal = terminal::Terminal::new(io::stdin());

    loop {
        if client.poll() {
            continue;
        }

        if let Some(mut input) = terminal.read() {
            input.pop(); // trim trailing newline
            client.send_command_raw(input)?;
        }

        thread::sleep(Duration::from_millis(100));
    }

    //Ok(())
}
