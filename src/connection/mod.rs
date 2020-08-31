pub use self::types::{Command, Reply, ReplyType};
use std::io;
use std::io::prelude::*;
use std::net;

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
                    // Truncate at the first control character (ie. CR/LF)
                    buffer.truncate(buffer.find(char::is_control).unwrap_or(buffer.len()));
                    split_server_name(&mut buffer);

                    if let Ok(command) = buffer.parse() {
                        println!("\x1B[94m<C {:?}\x1B[0m", command);
                        Some(Message::Command(command))
                    } else if let Ok(reply) = buffer.parse::<Reply>() {
                        println!("\x1B[92m<R {:?}\x1B[0m", reply);
                        Some(Message::Reply(reply.reply_type, reply.reply_message))
                    } else {
                        println!("\x1B[91m<? {}\x1B[0m", buffer);
                        None
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

pub enum Message {
    Command(Command),
    Reply(ReplyType, String),
}

fn split_server_name(raw_message: &mut String) -> Option<String> {
    if raw_message.starts_with(':') {
        let slice_index = raw_message.find(' ')?;
        let server_name = raw_message[1..slice_index].to_string();
        raw_message.replace_range(..slice_index + 1, "");
        Some(server_name)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_server_name_with_name() {
        let mut command =
            ":irc.example.net 001 foo :Welcome to the Internet Relay Network".to_string();
        let result = split_server_name(&mut command);

        assert_eq!("001 foo :Welcome to the Internet Relay Network", command);
        assert_eq!(Some("irc.example.net".to_string()), result);
    }

    #[test]
    fn split_server_name_no_name() {
        let mut command = "001 foo :Welcome to the Internet Relay Network".to_string();
        let result = split_server_name(&mut command);

        assert_eq!("001 foo :Welcome to the Internet Relay Network", command);
        assert_eq!(None, result);
    }

    #[test]
    fn split_server_name_missing_colon() {
        let mut command =
            "irc.example.net 001 foo :Welcome to the Internet Relay Network".to_string();
        let result = split_server_name(&mut command);

        assert_eq!(
            "irc.example.net 001 foo :Welcome to the Internet Relay Network",
            command
        );
        assert_eq!(None, result);
    }

    #[test]
    fn split_server_name_server_only() {
        let mut command = ":irc.example.net".to_string();
        let result = split_server_name(&mut command);

        assert_eq!(":irc.example.net", command);
        assert_eq!(None, result);
    }

    #[test]
    fn split_server_name_trailing_space() {
        let mut command = ":irc.example.net ".to_string();
        let result = split_server_name(&mut command);

        assert_eq!("", command);
        assert_eq!(Some("irc.example.net".to_string()), result);
    }
}
