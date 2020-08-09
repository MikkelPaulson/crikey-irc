use std::io;
use std::io::prelude::*;
use std::net;
use std::str::FromStr;

pub struct Connection<'a> {
    reader: Box<dyn 'a + io::BufRead>,
    writer: Box<dyn 'a + Write>,
}

impl Connection<'_> {
    pub fn new(stream: &net::TcpStream) -> Connection {
        stream.set_nonblocking(true).unwrap();

        let reader = io::BufReader::new(stream);

        Connection {
            reader: Box::new(reader),
            writer: Box::new(stream),
        }
    }

    pub fn poll(&mut self) -> Option<String> {
        let mut buffer = String::new();

        match self.reader.read_line(&mut buffer) {
            Ok(len) => {
                if len == 0 {
                    panic!("Stream disconnected");
                } else {
                    print!("< {}", buffer);
                    Some(buffer)
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => None,
            Err(e) => panic!("IO error: {}", e),
        }
    }

    pub fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        let raw_command = command_to_raw(command);
        self.send_command_raw(raw_command)
    }

    pub fn send_command_raw(&mut self, mut raw_command: String) -> std::io::Result<()> {
        raw_command.push_str("\r\n");
        print!("> {}", raw_command);
        self.writer.write(raw_command.as_bytes())?;
        Ok(())
    }
}

fn raw_to_command(raw_command: String) -> Option<Command> {
    let command_parts: Vec<&str> = raw_command.split(' ').collect();

    if command_parts.first()?.starts_with(':') {
        let (_, command_parts) = command_parts.split_at(1);
    }

    match command_parts.first()? {
        &"PASS" => {
            if command_parts.len() == 2 {
                Some(Command::Pass {
                    password: String::from(command_parts[1]),
                })
            } else {
                None
            }
        }
        &"NICK" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Nick {
                    nickname: String::from(command_parts[1]),
                    hopcount: match command_parts.get(2) {
                        Some(n) => u8::from_str(n).ok(),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        &"USER" => {
            if command_parts.len() >= 5 {
                Some(Command::User {
                    username: String::from(command_parts[1]),
                    hostname: String::from(command_parts[2]),
                    servername: String::from(command_parts[3]),
                    realname: String::from(command_parts[4..].join(" ").strip_prefix(":")?),
                })
            } else {
                None
            }
        }
        &"PING" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Ping {
                    server1: String::from(command_parts[1]),
                    server2: match command_parts.get(2) {
                        Some(&server2) => Some(String::from(server2)),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        &"PONG" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Pong {
                    daemon1: String::from(command_parts[1]),
                    daemon2: match command_parts.get(2) {
                        Some(&daemon2) => Some(String::from(daemon2)),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn command_to_raw(command: Command) -> String {
    match command {
        Command::Pass { password } => format!("PASS {}", password),
        Command::Nick { nickname, hopcount } => match hopcount {
            Some(hopcount) => format!("NICK {} {}", nickname, hopcount),
            None => format!("NICK {}", nickname),
        },
        Command::User {
            username,
            hostname,
            servername,
            realname,
        } => format!(
            "USER {} {} {} :{}",
            username, hostname, servername, realname
        ),
        Command::Ping { server1, server2 } => match server2 {
            Some(server2) => format!("PING {} {}", server1, server2),
            None => format!("PING {}", server1),
        },
        Command::Pong { daemon1, daemon2 } => match daemon2 {
            Some(daemon2) => format!("PONG {} {}", daemon1, daemon2),
            None => format!("PONG {}", daemon1),
        },
    }
}

#[derive(Debug)]
pub enum Command {
    // Connection registration
    Pass {
        password: String,
    },
    Nick {
        nickname: String,
        hopcount: Option<u8>,
    },
    User {
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    },
    //Oper { user: String, password: String },
    //Quit { message: Option<String> },

    // Channel operations
    //Join { channels: Vec<String>, keys: Vec<String> },
    //Part { channels: Vec<String> },

    // Sending messages
    //Privmsg { receivers: Vec<Messageable>, message: String },
    //Notice { receivers: Vec<Messageable>, message: String },

    // Miscellaneous messages
    Ping {
        server1: String,
        server2: Option<String>,
    },
    Pong {
        daemon1: String,
        daemon2: Option<String>,
    },
}

enum CommandType {
    // Connection registration
    Pass,
    Nick,
    User,
    //Oper,
    //Quit,

    // Channel operations
    //Join,
    //Part,

    // Sending messages
    //Privmsg,
    //Notice,

    // Miscellaneous messages
    Ping,
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_to_raw_pass() {
        assert_eq!(
            "PASS mysecretpass",
            command_to_raw(Command::Pass {
                password: String::from("mysecretpass"),
            }),
        );
    }

    #[test]
    fn command_to_raw_nick() {
        assert_eq!(
            "NICK potato",
            command_to_raw(Command::Nick {
                nickname: String::from("potato"),
                hopcount: None,
            }),
        );
        assert_eq!(
            "NICK carrot 5",
            command_to_raw(Command::Nick {
                nickname: String::from("carrot"),
                hopcount: Some(5),
            }),
        );
    }

    #[test]
    fn command_to_raw_user() {
        assert_eq!(
            "USER ab cd ef :gh ij",
            command_to_raw(Command::User {
                username: String::from("ab"),
                hostname: String::from("cd"),
                servername: String::from("ef"),
                realname: String::from("gh ij"),
            }),
        );
    }

    #[test]
    fn command_to_raw_ping() {
        assert_eq!(
            "PING myserver",
            command_to_raw(Command::Ping {
                server1: String::from("myserver"),
                server2: None,
            }),
        );
        assert_eq!(
            "PING myserver myotherserver",
            command_to_raw(Command::Ping {
                server1: String::from("myserver"),
                server2: Some(String::from("myotherserver")),
            }),
        );
    }

    #[test]
    fn command_to_raw_pong() {
        assert_eq!(
            "PONG myclient",
            command_to_raw(Command::Pong {
                daemon1: String::from("myclient"),
                daemon2: None,
            }),
        );
        assert_eq!(
            "PONG myclient myotherclient",
            command_to_raw(Command::Pong {
                daemon1: String::from("myclient"),
                daemon2: Some(String::from("myotherclient")),
            }),
        );
    }

    #[test]
    fn raw_to_command_pass() {
        let command = raw_to_command(String::from("PASS mysecretpass"));
        if let Some(Command::Pass { password }) = command {
            assert_eq!("mysecretpass", password);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn raw_to_command_nick() {
        let command = raw_to_command(String::from("NICK somebody"));
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("somebody", nickname);
            assert_eq!(None, hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command(String::from("NICK anybody 5"));
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("anybody", nickname);
            assert_eq!(Some(5), hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command(String::from("NICK anybody potato"));
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("anybody", nickname);
            assert_eq!(None, hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn raw_to_command_user() {
        assert!(raw_to_command(String::from("USER pjohnson local remote")).is_none());
        assert!(raw_to_command(String::from("USER pjohnson local remote realname")).is_none());
        assert!(raw_to_command(String::from("USER pjohnson local :remote realname")).is_none());

        let command = raw_to_command(String::from("USER pjohnson local remote :Potato Johnson"));
        if let Some(Command::User {
            username,
            hostname,
            servername,
            realname,
        }) = command
        {
            assert_eq!("pjohnson", username);
            assert_eq!("local", hostname);
            assert_eq!("remote", servername);
            assert_eq!("Potato Johnson", realname);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn raw_to_command_ping() {
        let command = raw_to_command(String::from("PING myserver"));
        if let Some(Command::Ping { server1, server2 }) = command {
            assert_eq!("myserver", server1);
            assert!(server2.is_none());
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command(String::from("PING myserver myotherserver"));
        if let Some(Command::Ping { server1, server2 }) = command {
            assert_eq!("myserver", server1);
            assert_eq!(Some(String::from("myotherserver")), server2);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        assert!(raw_to_command(String::from("PING")).is_none());
        assert!(raw_to_command(String::from("PING a b c")).is_none());
    }
}
