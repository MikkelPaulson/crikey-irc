use super::{Command, ParseError, ReplyType, Sender};
use std::result::Result;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Message {
    pub sender: Option<Sender>,
    pub body: MessageBody,
}

impl FromStr for Message {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let raw = raw.trim_end_matches(&['\r', '\n'][..]);
        let (sender, raw_body) = if raw.starts_with(':') && raw.contains(' ') {
            let index = raw.find(' ').unwrap();
            (Some(raw[1..index].parse()?), &raw[index + 1..])
        } else {
            (None, raw)
        };

        Ok(Message {
            sender,
            body: raw_body.parse()?,
        })
    }
}

impl From<Message> for String {
    fn from(message: Message) -> String {
        if let Some(sender) = message.sender {
            let mut result = String::from(":");
            result.push_str(&String::from(sender));
            result.push(' ');
            result.push_str(&String::from(message.body));
            result
        } else {
            String::from(message.body)
        }
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<Message>().is_err());
        assert!("🥔️".parse::<Message>().is_err());
        assert!(":abc".parse::<Message>().is_err());
        assert!(":abc ".parse::<Message>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Message {
                sender: Some("me".parse().unwrap()),
                body: MessageBody::Reply(ReplyType::PrvWelcome, "Hi there".to_string())
            }),
            ":me 001 Hi there\r\n".parse::<Message>()
        );
        assert_eq!(
            Ok(Message {
                sender: None,
                body: MessageBody::Command(Command::Nick {
                    nickname: "me".parse().unwrap(),
                })
            }),
            "NICK me\n".parse::<Message>()
        );
    }

    #[test]
    fn string() {
        assert_eq!(
            ":me 001 Hi there".to_string(),
            String::from(Message {
                sender: Some("me".parse().unwrap()),
                body: MessageBody::Reply(ReplyType::PrvWelcome, "Hi there".to_string())
            })
        );
        assert_eq!(
            "NICK me".to_string(),
            String::from(Message {
                sender: None,
                body: MessageBody::Command(Command::Nick {
                    nickname: "me".parse().unwrap(),
                })
            })
        );
    }
}

#[derive(PartialEq, Debug)]
pub enum MessageBody {
    Command(Command),
    Reply(ReplyType, String),
}

impl FromStr for MessageBody {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.chars().nth(0) {
            Some(c) if c.is_ascii_uppercase() => Ok(MessageBody::Command(raw.parse()?)),
            Some(c) if c.is_ascii_digit() => {
                if let Some(index) = raw.find(' ') {
                    Ok(MessageBody::Reply(
                        raw[..index].parse()?,
                        raw[index + 1..].to_string(),
                    ))
                } else {
                    Ok(MessageBody::Reply(raw.parse()?, String::new()))
                }
            }
            _ => Err(ParseError::new("MessageBody")),
        }
    }
}

impl From<MessageBody> for String {
    fn from(message_body: MessageBody) -> String {
        match message_body {
            MessageBody::Command(command) => String::from(command),
            MessageBody::Reply(reply_type, reply_body) => {
                let mut result = String::from(reply_type);
                result.push(' ');
                result.push_str(&String::from(reply_body));
                result
            }
        }
    }
}

#[cfg(test)]
mod message_body_tests {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<MessageBody>().is_err());
        assert!("🥔️".parse::<MessageBody>().is_err());
        assert!("00A ".parse::<MessageBody>().is_err());
        assert!("00A def".parse::<MessageBody>().is_err());
        assert!(":me 001 Hi there".parse::<MessageBody>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(MessageBody::Reply(ReplyType::PrvWelcome, "".to_string())),
            "001".parse::<MessageBody>()
        );
        assert_eq!(
            Ok(MessageBody::Reply(ReplyType::PrvWelcome, "".to_string())),
            "001 ".parse::<MessageBody>()
        );
        assert_eq!(
            Ok(MessageBody::Reply(
                ReplyType::PrvWelcome,
                "Hi there".to_string()
            )),
            "001 Hi there".parse::<MessageBody>()
        );
        assert_eq!(
            Ok(MessageBody::Command(Command::Nick {
                nickname: "me".parse().unwrap(),
            })),
            "NICK me".parse::<MessageBody>()
        );
    }

    #[test]
    fn string() {
        assert_eq!(
            "001 Hi there".to_string(),
            String::from(MessageBody::Reply(
                ReplyType::PrvWelcome,
                "Hi there".to_string()
            ))
        );
        assert_eq!(
            "NICK me".to_string(),
            String::from(MessageBody::Command(Command::Nick {
                nickname: "me".parse().unwrap(),
            }))
        );
    }
}
