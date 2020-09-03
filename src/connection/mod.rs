pub use self::message::{Command, Message, MessageBody, Reply, ReplyType};
pub use self::types::{Nickname, ParseError, Sender, Username};
use std::io;
use std::io::prelude::*;
use std::net;

mod message;
mod types;

pub struct Connection {
    reader: Box<dyn io::BufRead>,
    writer: Box<dyn io::Write>,
}

impl Connection {
    pub fn new(stream: net::TcpStream) -> Connection {
        stream.set_nonblocking(true).unwrap();
        let reader = io::BufReader::new(stream.try_clone().unwrap());

        Connection {
            reader: Box::new(reader),
            writer: Box::new(stream),
        }
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
