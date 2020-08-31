use super::ParseError;
use std::result::Result;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
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

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let command_parts: Vec<&str> = raw.split(' ').collect();

        match command_parts
            .first()
            .ok_or_else(|| ParseError::new("Command"))?
            .as_ref()
        {
            "PASS" => {
                if command_parts.len() == 2 {
                    Ok(Command::Pass {
                        password: command_parts[1].to_string(),
                    })
                } else {
                    Err(ParseError::new("Command"))
                }
            }
            "NICK" => {
                if command_parts.len() >= 2 && command_parts.len() <= 3 {
                    Ok(Command::Nick {
                        nickname: command_parts[1].to_string(),
                        hopcount: match command_parts.get(2) {
                            Some(n) => u8::from_str(n).ok(),
                            None => None,
                        },
                    })
                } else {
                    Err(ParseError::new("Command"))
                }
            }
            "USER" => {
                if command_parts.len() >= 5 {
                    Ok(Command::User {
                        username: command_parts[1].to_string(),
                        mode: u8::from_str(command_parts[2]).unwrap_or(0),
                        // part 3 is unused
                        realname: command_parts[4..]
                            .join(" ")
                            .strip_prefix(":")
                            .ok_or_else(|| ParseError::new("Command"))?
                            .to_string(),
                    })
                } else {
                    Err(ParseError::new("Command"))
                }
            }
            "PING" => match command_parts.len() {
                1 => Ok(Command::Ping {
                    to: None,
                    from: None,
                }),
                2 => {
                    if command_parts[1].starts_with(':') {
                        Ok(Command::Ping {
                            to: None,
                            from: command_parts[1].get(1..).map(|s| s.to_string()),
                        })
                    } else {
                        Ok(Command::Ping {
                            to: Some(command_parts[1].to_string()),
                            from: None,
                        })
                    }
                }
                3 => Ok(Command::Ping {
                    to: Some(command_parts[2].to_string()),
                    from: Some(command_parts[1].to_string()),
                }),
                _ => Err(ParseError::new("Command")),
            },
            "PONG" => match command_parts.len() {
                2 => Ok(Command::Pong {
                    to: None,
                    from: command_parts[1].to_string(),
                }),
                3 => Ok(Command::Pong {
                    to: Some(command_parts[2].to_string()),
                    from: command_parts[1].to_string(),
                }),
                _ => Err(ParseError::new("Command")),
            },
            _ => Err(ParseError::new("Command")),
        }
    }
}

impl From<Command> for String {
    fn from(command: Command) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_from_pass() {
        assert_eq!(
            "PASS mysecretpass".to_string(),
            String::from(Command::Pass {
                password: "mysecretpass".to_string(),
            }),
        );
    }

    #[test]
    fn string_from_nick() {
        assert_eq!(
            "NICK potato".to_string(),
            String::from(Command::Nick {
                nickname: "potato".to_string(),
                hopcount: None,
            }),
        );
        assert_eq!(
            "NICK carrot 5".to_string(),
            String::from(Command::Nick {
                nickname: "carrot".to_string(),
                hopcount: Some(5),
            }),
        );
    }

    #[test]
    fn string_from_user() {
        assert_eq!(
            "USER pjohnson 0 * :Potato Johnson".to_string(),
            String::from(Command::User {
                username: "pjohnson".to_string(),
                mode: 0,
                realname: "Potato Johnson".to_string(),
            }),
        );
    }

    #[test]
    fn string_from_ping() {
        assert_eq!(
            "PING".to_string(),
            String::from(Command::Ping {
                to: None,
                from: None
            }),
        );
        assert_eq!(
            "PING :me".to_string(),
            String::from(Command::Ping {
                to: None,
                from: Some("me".to_string()),
            }),
        );
        assert_eq!(
            "PING myserver".to_string(),
            String::from(Command::Ping {
                to: Some("myserver".to_string()),
                from: None
            }),
        );
        assert_eq!(
            "PING me myserver".to_string(),
            String::from(Command::Ping {
                to: Some("myserver".to_string()),
                from: Some("me".to_string()),
            }),
        );
    }

    #[test]
    fn string_from_pong() {
        assert_eq!(
            "PONG me".to_string(),
            String::from(Command::Pong {
                from: "me".to_string(),
                to: None,
            }),
        );
        assert_eq!(
            "PONG me myserver".to_string(),
            String::from(Command::Pong {
                from: "me".to_string(),
                to: Some("myserver".to_string()),
            }),
        );
    }

    #[test]
    fn pass_from_string() {
        assert_eq!(
            Ok(Command::Pass {
                password: "mysecretpass".to_string()
            }),
            "PASS mysecretpass".parse::<Command>()
        );
    }

    #[test]
    fn nick_from_string() {
        assert_eq!(
            Ok(Command::Nick {
                nickname: "somebody".to_string(),
                hopcount: None
            }),
            "NICK somebody".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Nick {
                nickname: "anybody".to_string(),
                hopcount: Some(5)
            }),
            "NICK anybody 5".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Nick {
                nickname: "anybody".to_string(),
                hopcount: None
            }),
            "NICK anybody potato".parse::<Command>()
        );
    }

    #[test]
    fn user_from_string() {
        assert!("USER pjohnson 0 *".parse::<Command>().is_err());
        assert!("USER pjohnson 0 * realname".parse::<Command>().is_err());

        assert_eq!(
            Ok(Command::User {
                username: "pjohnson".to_string(),
                mode: 0,
                realname: "Potato Johnson".to_string()
            }),
            "USER pjohnson 0 * :Potato Johnson".parse::<Command>()
        );
    }

    #[test]
    fn ping_from_string() {
        assert_eq!(
            Ok(Command::Ping {
                to: None,
                from: None
            }),
            "PING".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Ping {
                to: Some("myserver".to_string()),
                from: None
            }),
            "PING myserver".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Ping {
                to: Some("myserver".to_string()),
                from: Some("me".to_string())
            }),
            "PING me myserver".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Ping {
                to: None,
                from: Some("me".to_string())
            }),
            "PING :me".parse::<Command>()
        );
        assert!("PING a b c".parse::<Command>().is_err());
    }

    #[test]
    fn pong_from_string() {
        assert_eq!(
            Ok(Command::Pong {
                to: None,
                from: "me".to_string()
            }),
            "PONG me".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Pong {
                to: Some("myserver".to_string()),
                from: "me".to_string()
            }),
            "PONG me myserver".parse::<Command>()
        );
        assert!("PONG".parse::<Command>().is_err());
        assert!("PONG a b c".parse::<Command>().is_err());
    }
}
