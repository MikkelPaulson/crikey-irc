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
        let connection = Connection::connect(stream);
        let mut client = Client {
            connection,
            auth_token,
        };
        client.authenticate();
        client
    }

    fn authenticate(&mut self) {
        if let Some(command) = self.auth_token.pass() {
            self.connection
                .send_command(command)
                .expect("Could not authenticate with server.");
        }

        self.connection
            .send_command(self.auth_token.nick())
            .expect("Could not authenticate with server.");
        self.connection
            .send_command(self.auth_token.user())
            .expect("Could not authenticate with server.");
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

#[cfg(test)]
mod test_client {
    use super::*;
    use pipe::pipe;
    use std::io::prelude::*;
    use std::thread::spawn;

    fn get_token(password: Option<String>) -> AuthToken {
        AuthToken {
            nickname: "spudly".parse().unwrap(),
            username: "pjohnson".parse().unwrap(),
            mode: 0,
            realname: "Potato Johnson".to_string(),
            password,
        }
    }

    fn spawn_client(
        auth_token: AuthToken,
        client_callback: fn(Client),
    ) -> (pipe::PipeReader, pipe::PipeWriter) {
        let (input_pipe_read, input_pipe_write) = pipe();
        let (output_pipe_read, output_pipe_write) = pipe();

        spawn(move || {
            let connection =
                Connection::new(Box::new(input_pipe_read), Box::new(output_pipe_write));
            let client = Client {
                connection,
                auth_token,
            };
            client_callback(client);
        });

        (output_pipe_read, input_pipe_write)
    }

    #[test]
    fn authenticate_without_password() {
        let (mut reader, _) = spawn_client(get_token(None), |mut client| client.authenticate());

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("NICK spudly\r\n".to_string(), buffer);

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("USER pjohnson 0 * :Potato Johnson\r\n".to_string(), buffer);

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("".to_string(), buffer);
    }

    #[test]
    fn authenticate_with_password() {
        let (mut reader, _) =
            spawn_client(get_token(Some("secretpass".to_string())), |mut client| {
                client.authenticate()
            });

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("PASS secretpass\r\n".to_string(), buffer);

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("NICK spudly\r\n".to_string(), buffer);

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("USER pjohnson 0 * :Potato Johnson\r\n".to_string(), buffer);

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("".to_string(), buffer);
    }

    #[test]
    fn responds_to_ping() {
        let (mut reader, mut writer) = spawn_client(get_token(None), |mut client| {
            client.poll();
        });
        write!(writer, "PING irc.example.com spudly\r\n").unwrap();

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        assert_eq!("PONG spudly irc.example.com\r\n", buffer);
    }
}

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod test_auth_token {
    use super::*;

    fn get_token(password: Option<String>) -> AuthToken {
        AuthToken {
            nickname: "spudly".parse().unwrap(),
            username: "pjohnson".parse().unwrap(),
            mode: 0,
            realname: "Potato Johnson".to_string(),
            password,
        }
    }

    #[test]
    fn pass_none() {
        let auth_token = get_token(None);
        assert_eq!(None, auth_token.pass());
    }

    #[test]
    fn pass_some() {
        let auth_token = get_token(Some("secretpass".to_string()));
        assert_eq!(
            Some(Command::Pass {
                password: "secretpass".to_string()
            }),
            auth_token.pass()
        );
    }

    #[test]
    fn nick() {
        let auth_token = get_token(None);
        assert_eq!(
            Command::Nick {
                nickname: "spudly".parse().unwrap()
            },
            auth_token.nick()
        );
    }

    #[test]
    fn user() {
        let auth_token = get_token(None);
        assert_eq!(
            Command::User {
                username: "pjohnson".parse().unwrap(),
                mode: 0,
                realname: "Potato Johnson".parse().unwrap()
            },
            auth_token.user()
        );
    }
}
