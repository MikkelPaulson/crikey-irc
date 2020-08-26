use crate::connection::{self, Command, Connection};
use std::io;
use std::net;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub fn connect<T: net::ToSocketAddrs>(addr: T, token: AuthToken) -> Client {
        let stream = net::TcpStream::connect(addr).expect("Could not connect to server.");
        let mut connection = Connection::new(stream);

        if let Some(command) = token.pass() {
            connection
                .send_command(command)
                .expect("Could not authenticate with server.");
        }

        connection
            .send_command(token.nick())
            .expect("Could not authenticate with server.");
        connection
            .send_command(token.user())
            .expect("Could not authenticate with server.");

        Client::new(connection)
    }

    fn new(connection: Connection) -> Client {
        Client { connection }
    }

    pub fn poll(&mut self) -> bool {
        match self.connection.poll() {
            Some(connection::Message::Command(_command)) => true,
            Some(connection::Message::Reply(_reply_type, _reply_body)) => true,
            None => false,
        }
    }

    pub fn send_command_raw(&mut self, raw_command: String) -> io::Result<()> {
        self.connection.send_command_raw(raw_command)
    }
}

pub struct AuthToken {
    pub nickname: String,
    pub username: String,
    pub mode: u8,
    pub realname: String,
    pub password: Option<String>,
}

impl AuthToken {
    fn pass(&self) -> Option<Command> {
        match &self.password {
            Some(password) => Some(Command::Pass {
                password: password.to_owned(),
            }),
            None => None,
        }
    }

    fn nick(&self) -> Command {
        Command::Nick {
            nickname: self.nickname.to_owned(),
            hopcount: None,
        }
    }

    fn user(&self) -> Command {
        Command::User {
            username: self.username.to_owned(),
            mode: self.mode,
            realname: self.realname.to_owned(),
        }
    }
}
