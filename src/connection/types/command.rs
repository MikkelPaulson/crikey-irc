use std::str::FromStr;

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
        mode: u8,
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
        from: Option<String>,
        to: Option<String>,
    },
    Pong {
        from: String,
        to: Option<String>,
    },
}

pub fn from_raw(raw_command: &str) -> Option<Command> {
    let command_parts: Vec<&str> = raw_command.split(' ').collect();

    match command_parts.first()?.as_ref() {
        "PASS" => {
            if command_parts.len() == 2 {
                Some(Command::Pass {
                    password: command_parts[1].to_string(),
                })
            } else {
                None
            }
        }
        "NICK" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Nick {
                    nickname: command_parts[1].to_string(),
                    hopcount: match command_parts.get(2) {
                        Some(n) => u8::from_str(n).ok(),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        "USER" => {
            if command_parts.len() >= 5 {
                Some(Command::User {
                    username: command_parts[1].to_string(),
                    mode: u8::from_str(command_parts[2]).unwrap_or(0),
                    // part 3 is unused
                    realname: command_parts[4..].join(" ").strip_prefix(":")?.to_string(),
                })
            } else {
                None
            }
        }
        "PING" => match command_parts.len() {
            1 => Some(Command::Ping {
                to: None,
                from: None,
            }),
            2 => {
                if command_parts[1].starts_with(':') {
                    Some(Command::Ping {
                        to: None,
                        from: command_parts[1].get(1..).map(|s| s.to_string()),
                    })
                } else {
                    Some(Command::Ping {
                        to: Some(command_parts[1].to_string()),
                        from: None,
                    })
                }
            }
            3 => Some(Command::Ping {
                to: Some(command_parts[2].to_string()),
                from: Some(command_parts[1].to_string()),
            }),
            _ => None,
        },
        "PONG" => match command_parts.len() {
            2 => Some(Command::Pong {
                to: None,
                from: command_parts[1].to_string(),
            }),
            3 => Some(Command::Pong {
                to: Some(command_parts[2].to_string()),
                from: command_parts[1].to_string(),
            }),
            _ => None,
        },
        _ => None,
    }
}

pub fn to_raw(command: Command) -> String {
    match command {
        Command::Pass { password } => format!("PASS {}", password),
        Command::Nick { nickname, hopcount } => match hopcount {
            Some(hopcount) => format!("NICK {} {}", nickname, hopcount),
            None => format!("NICK {}", nickname),
        },
        Command::User {
            username,
            mode,
            realname,
        } => format!("USER {} {} * :{}", username, mode, realname),
        Command::Ping { to, from } => match (to, from) {
            (Some(to), Some(from)) => format!("PING {} {}", from, to),
            (Some(to), None) => format!("PING {}", to),
            (None, Some(from)) => format!("PING :{}", from),
            (None, None) => "PING".to_string(),
        },
        Command::Pong { to, from } => match to {
            Some(to) => format!("PONG {} {}", from, to),
            None => format!("PONG {}", from),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_raw_pass() {
        assert_eq!(
            "PASS mysecretpass",
            to_raw(Command::Pass {
                password: "mysecretpass".to_string(),
            }),
        );
    }

    #[test]
    fn to_raw_nick() {
        assert_eq!(
            "NICK potato",
            to_raw(Command::Nick {
                nickname: "potato".to_string(),
                hopcount: None,
            }),
        );
        assert_eq!(
            "NICK carrot 5",
            to_raw(Command::Nick {
                nickname: "carrot".to_string(),
                hopcount: Some(5),
            }),
        );
    }

    #[test]
    fn to_raw_user() {
        assert_eq!(
            "USER pjohnson 0 * :Potato Johnson",
            to_raw(Command::User {
                username: "pjohnson".to_string(),
                mode: 0,
                realname: "Potato Johnson".to_string(),
            }),
        );
    }

    #[test]
    fn to_raw_ping() {
        assert_eq!(
            "PING",
            to_raw(Command::Ping {
                to: None,
                from: None
            }),
        );
        assert_eq!(
            "PING :me",
            to_raw(Command::Ping {
                to: None,
                from: Some("me".to_string()),
            }),
        );
        assert_eq!(
            "PING myserver",
            to_raw(Command::Ping {
                to: Some("myserver".to_string()),
                from: None
            }),
        );
        assert_eq!(
            "PING me myserver",
            to_raw(Command::Ping {
                to: Some("myserver".to_string()),
                from: Some("me".to_string()),
            }),
        );
    }

    #[test]
    fn to_raw_pong() {
        assert_eq!(
            "PONG me",
            to_raw(Command::Pong {
                from: "me".to_string(),
                to: None,
            }),
        );
        assert_eq!(
            "PONG me myserver",
            to_raw(Command::Pong {
                from: "me".to_string(),
                to: Some("myserver".to_string()),
            }),
        );
    }

    #[test]
    fn from_raw_pass() {
        let command = from_raw("PASS mysecretpass");
        if let Some(Command::Pass { password }) = command {
            assert_eq!("mysecretpass", password);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn from_raw_nick() {
        let command = from_raw("NICK somebody");
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("somebody", nickname);
            assert_eq!(None, hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = from_raw("NICK anybody 5");
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("anybody", nickname);
            assert_eq!(Some(5), hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = from_raw("NICK anybody potato");
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("anybody", nickname);
            assert_eq!(None, hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn from_raw_user() {
        assert!(from_raw("USER pjohnson 0 *").is_none());
        assert!(from_raw("USER pjohnson 0 * realname").is_none());

        let command = from_raw("USER pjohnson 0 * :Potato Johnson");
        if let Some(Command::User {
            username,
            mode,
            realname,
        }) = command
        {
            assert_eq!("pjohnson", username);
            assert_eq!(0, mode);
            assert_eq!("Potato Johnson", realname);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn from_raw_ping() {
        let command = from_raw("PING");
        if let Some(Command::Ping { to, from }) = command {
            assert!(to.is_none());
            assert!(from.is_none());
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = from_raw("PING myserver");
        if let Some(Command::Ping { to, from }) = command {
            assert_eq!(Some("myserver".to_string()), to);
            assert!(from.is_none());
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = from_raw("PING me myserver");
        if let Some(Command::Ping { to, from }) = command {
            assert_eq!(Some("myserver".to_string()), to);
            assert_eq!(Some("me".to_string()), from);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = from_raw("PING :me");
        if let Some(Command::Ping { to, from }) = command {
            assert!(to.is_none());
            assert_eq!(Some("me".to_string()), from);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        assert!(from_raw("PING a b c").is_none());
    }

    #[test]
    fn from_raw_pong() {
        let command = from_raw("PONG me");
        if let Some(Command::Pong { to, from }) = command {
            assert_eq!("me".to_string(), from);
            assert!(to.is_none());
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = from_raw("PONG me myserver");
        if let Some(Command::Pong { to, from }) = command {
            assert_eq!("me".to_string(), from);
            assert_eq!(Some("myserver".to_string()), to);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        assert!(from_raw("PONG").is_none());
        assert!(from_raw("PONG a b c").is_none());
    }
}
