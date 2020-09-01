pub use self::channel::{Channel, ChannelID, ChannelName, ChannelType};
pub use self::command::Command;
pub use self::errors::ParseError;
pub use self::host::{Host, Hostname, IpAddr, Ipv4Addr, Ipv6Addr, Servername};
pub use self::key::Key;
pub use self::keyword_list::KeywordList;
pub use self::message::{Message, MessageBody};
pub use self::msg_target::{Recipient, Sender};
pub use self::nickname::Nickname;
pub use self::reply::{Reply, ReplyType};
pub use self::target_mask::{HostMask, ServerMask, TargetMask};
pub use self::user::User;

use std::iter::IntoIterator;
use std::ops::Index;
use std::result::Result;
use std::str::FromStr;
use std::vec::IntoIter;

mod channel;
mod command;
mod errors;
mod host;
mod key;
mod keyword_list;
mod message;
mod msg_target;
mod nickname;
mod reply;
mod target_mask;
mod user;

#[derive(PartialEq, Debug)]
struct CommandArgs {
    args: Vec<String>,
    has_space: bool,
}

impl CommandArgs {
    pub fn new() -> CommandArgs {
        CommandArgs {
            args: Vec::new(),
            has_space: false,
        }
    }

    pub fn push(&mut self, value: String) -> Result<(), ParseError> {
        if self.args.len() >= 15 || self.has_space {
            return Err(ParseError::new("CommandArgs"));
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
}

impl Index<usize> for CommandArgs {
    type Output = String;

    fn index(&self, index: usize) -> &String {
        &self.args[index]
    }
}

impl IntoIterator for CommandArgs {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

impl FromStr for CommandArgs {
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

impl From<CommandArgs> for String {
    fn from(command_args: CommandArgs) -> String {
        let mut result = String::new();

        for arg in command_args.args {
            if !result.is_empty() {
                result.push(' ');
            }
            if arg.contains(' ') {
                result.push(':');
            }
            result.push_str(&arg);
        }
        result
    }
}

#[cfg(test)]
mod test_command_args {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(
            Ok(CommandArgs {
                has_space: false,
                args: Vec::new()
            }),
            "".parse::<CommandArgs>()
        );

        assert_eq!(
            Ok(CommandArgs {
                has_space: false,
                args: vec!["".to_string()]
            }),
            ":".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
                has_space: false,
                args: vec!["abc".to_string(), "def".to_string(),]
            }),
            "abc def".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
                has_space: true,
                args: vec!["a".to_string(), "b".to_string(), "c  d".to_string(),]
            }),
            " a  b  :c  d".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
                has_space: false,
                args: vec!["a:b".to_string(), "cd".to_string(),]
            }),
            "a:b :cd".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
                has_space: false,
                args: vec!["abc".to_string(), "".to_string(),]
            }),
            "abc :".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
                has_space: true,
                args: vec![" a b c".to_string(),]
            }),
            ": a b c".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
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
            "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16".parse::<CommandArgs>()
        );
        assert_eq!(
            Ok(CommandArgs {
                has_space: true,
                args: vec!["🥔️".to_string(), "🥔️ 🥔️".to_string(),]
            }),
            "🥔️ :🥔️ 🥔️".parse::<CommandArgs>()
        );
    }

    #[test]
    fn to_string() {
        let mut command_args = CommandArgs::new();
        command_args.push("abc".to_string()).unwrap();
        command_args.push("def".to_string()).unwrap();
        assert_eq!("abc def".to_string(), String::from(command_args));

        let mut command_args = CommandArgs::new();
        command_args.push("abc".to_string()).unwrap();
        command_args.push("def ghi".to_string()).unwrap();
        assert_eq!("abc :def ghi".to_string(), String::from(command_args));
    }

    #[test]
    fn push_validates() {
        let mut command_args = CommandArgs::new();
        for i in 1..=15 {
            command_args
                .push(i.to_string())
                .expect(&format!("No error expected on iteration {}", i));
        }
        assert!(command_args.push("16".to_string()).is_err());
        assert_eq!(15, command_args.len());

        let mut command_args = CommandArgs::new();
        command_args.push("nospaces".to_string()).unwrap();
        command_args.push("some spaces".to_string()).unwrap();
        assert!(command_args.push("more spaces".to_string()).is_err());
        assert!(command_args.push("nomorespaces".to_string()).is_err());
        assert_eq!(2, command_args.len());
    }
}
