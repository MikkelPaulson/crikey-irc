use crate::connection::{Command, Connection, Message, MessageBody, Nickname, ReplyType, Username};
use std::io;
use std::net;

pub struct Client {
    connection: Connection,
    auth_token: AuthToken,
}

impl Client {
    pub fn connect<T: net::ToSocketAddrs>(addr: T, auth_token: AuthToken) -> Client {
        let stream = net::TcpStream::connect(addr).expect("Could not connect to server.");
        let mut connection = Connection::new(stream);

        if let Some(command) = auth_token.pass() {
            connection
                .send_command(command)
                .expect("Could not authenticate with server.");
        }

        connection
            .send_command(auth_token.nick())
            .expect("Could not authenticate with server.");
        connection
            .send_command(auth_token.user())
            .expect("Could not authenticate with server.");

        Client::new(connection, auth_token)
    }

    fn new(connection: Connection, auth_token: AuthToken) -> Client {
        Client {
            connection,
            auth_token,
        }
    }

    pub fn poll(&mut self) -> bool {
        match self.connection.poll() {
            Some(Message {
                body: MessageBody::Command(command),
                ..
            }) => self.handle_command(command),
            Some(Message {
                body: MessageBody::Reply(reply_type, reply_body),
                ..
            }) => self.handle_reply(reply_type, reply_body),
            None => return false,
        }
        true
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Ping { .. } => self.handle_command_ping(command),
            _ => return,
        }
    }

    fn handle_command_ping(&mut self, command: Command) {
        if let Command::Ping { from, .. } = command {
            self.connection
                .send_command(Command::Pong {
                    to: from,
                    from: self.auth_token.nickname.clone().into(),
                })
                .ok();
        }
    }

    fn handle_reply(&self, _reply_type: ReplyType, _reply_body: String) {}

    pub fn send_command_raw(&mut self, raw_command: String) -> io::Result<()> {
        self.connection.send_command_raw(raw_command)
    }
}

pub struct AuthToken {
    pub nickname: Nickname,
    pub username: Username,
    pub mode: u8,
    pub realname: String,
    pub password: Option<String>,
}

impl AuthToken {
    fn pass(&self) -> Option<Command> {
        match &self.password {
            Some(password) => Some(Command::Pass {
                password: password.clone(),
            }),
            None => None,
        }
    }

    fn nick(&self) -> Command {
        Command::Nick {
            nickname: self.nickname.clone(),
        }
    }

    fn user(&self) -> Command {
        Command::User {
            username: self.username.clone(),
            mode: self.mode,
            realname: self.realname.clone(),
        }
    }
}
