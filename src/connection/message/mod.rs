pub use self::command::Command;
pub use self::reply::{Reply, ReplyType};
use super::{ParseError, Sender};
use std::iter::IntoIterator;
use std::ops::Index;
use std::result::Result;
use std::str::FromStr;
use std::vec::IntoIter;

mod command;
mod reply;

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
mod test_message {
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
                body: MessageBody::Reply(ReplyType::PrvWelcome, ":Hi there".parse().unwrap())
            }),
            ":me 001 :Hi there\r\n".parse::<Message>()
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
            ":me 001 :Hi there".to_string(),
            String::from(Message {
                sender: Some("me".parse().unwrap()),
                body: MessageBody::Reply(ReplyType::PrvWelcome, ":Hi there".parse().unwrap())
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
    Reply(ReplyType, MessageParams),
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
                        raw[index + 1..].parse()?,
                    ))
                } else {
                    Ok(MessageBody::Reply(raw.parse()?, MessageParams::new()))
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
mod test_message_body {
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
            Ok(MessageBody::Reply(
                ReplyType::PrvWelcome,
                "".parse().unwrap()
            )),
            "001".parse::<MessageBody>()
        );
        assert_eq!(
            Ok(MessageBody::Reply(
                ReplyType::PrvWelcome,
                "".parse().unwrap()
            )),
            "001 ".parse::<MessageBody>()
        );
        assert_eq!(
            Ok(MessageBody::Reply(
                ReplyType::PrvWelcome,
                ":Hi there".parse().unwrap()
            )),
            "001 :Hi there".parse::<MessageBody>()
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
            "001 :Hi there".to_string(),
            String::from(MessageBody::Reply(
                ReplyType::PrvWelcome,
                ":Hi there".parse().unwrap()
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

#[derive(PartialEq, Debug)]
pub struct MessageParams {
    args: Vec<String>,
    has_space: bool,
}

impl MessageParams {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            has_space: false,
        }
    }

    pub fn push(&mut self, value: String) -> Result<(), ParseError> {
        if self.args.len() >= 15 || self.has_space {
            return Err(ParseError::new("MessageParams"));
        } else if value.contains(' ') {
            self.has_space = true;
        }

        self.args.push(value);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.args.get(index)
    }

    pub fn to_string_with_prefix(self, prefix: &str) -> String {
        let mut result = String::from(prefix);
        result.push(' ');
        result.push_str(&String::from(self));
        result
    }
}

impl Index<usize> for MessageParams {
    type Output = String;

    fn index(&self, index: usize) -> &String {
        &self.args[index]
    }
}

impl IntoIterator for MessageParams {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

impl FromStr for MessageParams {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut args = Vec::<String>::new();
        let mut start = 0;

        for (end, _) in raw.match_indices(' ') {
            if raw[start..].starts_with(':') || args.len() >= 14 {
                break;
            }
            if start < end {
                args.push(raw[start..end].to_string());
            }
            start = end + 1;
        }

        if start < raw.len() {
            if raw[start..].starts_with(':') {
                start = start + 1;
            }
            args.push(raw[start..].to_string());
        }

        Ok(Self {
            has_space: args.last().map(|s| s.contains(' ')).unwrap_or(false),
            args,
        })
    }
}

impl From<MessageParams> for String {
    fn from(command_args: MessageParams) -> String {
        let mut result = String::new();
        let mut last_elment_is_empty = false;

        for arg in command_args.args {
            if !result.is_empty() {
                result.push(' ');
            }
            if arg.contains(' ') {
                result.push(':');
            }

            if arg == "" {
                result.push('*');
                last_elment_is_empty = true;
            } else {
                result.push_str(&arg);
                last_elment_is_empty = false;
            }
        }
        if last_elment_is_empty {
            result.pop();
            result.push(':');
        }
        result
    }
}

impl From<Vec<String>> for MessageParams {
    fn from(original: Vec<String>) -> Self {
        let mut message_params = Self::new();
        for arg in original.into_iter() {
            message_params.push(arg).unwrap();
        }
        message_params
    }
}

#[cfg(test)]
mod test_command_args {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(
            Ok(MessageParams {
                has_space: false,
                args: Vec::new()
            }),
            "".parse::<MessageParams>()
        );

        assert_eq!(
            Ok(MessageParams {
                has_space: false,
                args: vec!["".to_string()]
            }),
            ":".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: false,
                args: vec!["abc".to_string(), "def".to_string(),]
            }),
            "abc def".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: true,
                args: vec!["a".to_string(), "b".to_string(), "c  d".to_string(),]
            }),
            " a  b  :c  d".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: false,
                args: vec!["a:b".to_string(), "cd".to_string(),]
            }),
            "a:b :cd".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: false,
                args: vec!["abc".to_string(), "".to_string(),]
            }),
            "abc :".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: true,
                args: vec![" a b c".to_string(),]
            }),
            ": a b c".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: true,
                args: vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                    "8".to_string(),
                    "9".to_string(),
                    "10".to_string(),
                    "11".to_string(),
                    "12".to_string(),
                    "13".to_string(),
                    "14".to_string(),
                    "15 16".to_string(),
                ]
            }),
            "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16".parse::<MessageParams>()
        );
        assert_eq!(
            Ok(MessageParams {
                has_space: true,
                args: vec!["🥔️".to_string(), "🥔️ 🥔️".to_string(),]
            }),
            "🥔️ :🥔️ 🥔️".parse::<MessageParams>()
        );
    }

    #[test]
    fn to_string() {
        let mut command_args = MessageParams::new();
        command_args.push("abc".to_string()).unwrap();
        command_args.push("def".to_string()).unwrap();
        assert_eq!("abc def".to_string(), String::from(command_args));

        let mut command_args = MessageParams::new();
        command_args.push("abc".to_string()).unwrap();
        command_args.push("def ghi".to_string()).unwrap();
        assert_eq!("abc :def ghi".to_string(), String::from(command_args));
    }

    #[test]
    fn push_validates() {
        let mut command_args = MessageParams::new();
        for i in 1..=15 {
            command_args
                .push(i.to_string())
                .expect(&format!("No error expected on iteration {}", i));
        }
        assert!(command_args.push("16".to_string()).is_err());
        assert_eq!(15, command_args.len());

        let mut command_args = MessageParams::new();
        command_args.push("nospaces".to_string()).unwrap();
        command_args.push("some spaces".to_string()).unwrap();
        assert!(command_args.push("more spaces".to_string()).is_err());
        assert!(command_args.push("nomorespaces".to_string()).is_err());
        assert_eq!(2, command_args.len());
    }
}
