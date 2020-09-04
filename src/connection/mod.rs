pub use self::entity::{Nickname, Sender, Username};
pub use self::message::{Command, Message, MessageBody, Reply, ReplyType};
use std::error::Error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::net;

mod entity;
mod message;
mod syntax;

pub struct Connection {
    reader: Box<dyn io::BufRead>,
    writer: Box<dyn io::Write>,
}

impl Connection {
    pub fn connect(stream: net::TcpStream) -> Self {
        stream.set_nonblocking(true).unwrap();
        let reader = io::BufReader::new(stream.try_clone().unwrap());
        Self::new(Box::new(reader), Box::new(stream))
    }

    pub fn new(reader: Box<dyn io::BufRead>, writer: Box<dyn io::Write>) -> Self {
        Connection { reader, writer }
    }

    pub fn poll(&mut self) -> Option<Message> {
        let mut buffer = String::new();

        match self.reader.read_line(&mut buffer) {
            Ok(len) => {
                if len == 0 {
                    panic!("Stream disconnected");
                } else {
                    match buffer.parse::<Message>() {
                        Ok(message) => {
                            println!("\x1B[94m<< {:?}\x1B[0m", message);
                            Some(message)
                        }
                        Err(e) => {
                            print!("\x1B[91m<? {}\x1B[0m", buffer);
                            println!("\x1B[91m   {:?}\x1B[0m", e);
                            None
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => None,
            Err(e) => panic!("IO error: {}", e),
        }
    }

    pub fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        let raw_command = String::from(command);
        self.send_command_raw(raw_command)
    }

    pub fn send_command_raw(&mut self, mut raw_command: String) -> std::io::Result<()> {
        raw_command.push_str("\r\n");
        print!(">> {}", raw_command);
        self.writer.write(raw_command.as_bytes())?;
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseError(&'static str);

impl ParseError {
    pub fn new(struct_name: &'static str) -> Self {
        ParseError(struct_name)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse component: {}", self)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod test_connection {
    use super::*;
    use pipe::pipe;
    use std::io;
    use std::thread::spawn;

    #[test]
    #[should_panic(expected = "Stream disconnected")]
    fn poll_disconnected() {
        let null_read = io::empty();
        let null_write = io::sink();
        let buf_read = io::BufReader::new(null_read);
        let mut connection = Connection::new(Box::new(buf_read), Box::new(null_write));

        connection.poll().unwrap();
    }

    #[test]
    fn poll_command() {
        let (pipe_read, mut pipe_write) = pipe();
        let null_write = io::sink();
        let buf_read = io::BufReader::new(pipe_read);
        let mut connection = Connection::new(Box::new(buf_read), Box::new(null_write));

        spawn(move || write!(pipe_write, ":irc.example.com PING somebody\r\n"));

        assert_eq!(
            Some(Message {
                sender: Some("irc.example.com".parse().unwrap()),
                body: MessageBody::Command(Command::Ping {
                    to: Some("somebody".parse().unwrap()),
                    from: None
                })
            }),
            connection.poll()
        );
    }

    #[test]
    fn poll_unrecognized() {
        let (pipe_read, mut pipe_write) = pipe();
        let null_write = io::sink();
        let buf_read = io::BufReader::new(pipe_read);
        let mut connection = Connection::new(Box::new(buf_read), Box::new(null_write));

        spawn(move || write!(pipe_write, "potato\r\n"));

        assert_eq!(None, connection.poll());
    }

    #[test]
    fn send_command() {
        let (mut pipe_read, pipe_write) = pipe();

        spawn(move || {
            let buf_read = io::BufReader::new(io::empty());
            let mut connection = Connection::new(Box::new(buf_read), Box::new(pipe_write));
            connection
                .send_command(Command::Pong {
                    from: "somebody".parse().unwrap(),
                    to: None,
                })
                .unwrap();
        });

        let mut buffer = String::new();
        pipe_read.read_line(&mut buffer).unwrap();

        assert_eq!("PONG somebody\r\n".to_string(), buffer);
    }

    #[test]
    fn send_command_raw() {
        let (mut pipe_read, pipe_write) = pipe();

        spawn(move || {
            let buf_read = io::BufReader::new(io::empty());
            let mut connection = Connection::new(Box::new(buf_read), Box::new(pipe_write));
            connection
                .send_command_raw("hello dolly".to_string())
                .unwrap();
        });

        let mut buffer = String::new();
        pipe_read.read_line(&mut buffer).unwrap();

        assert_eq!("hello dolly\r\n".to_string(), buffer);
    }
}
