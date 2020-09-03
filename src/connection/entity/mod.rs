pub use self::channel::{Channel, ChannelKey};
pub use self::user::{Nickname, Username};
use super::types::{Host, Servername, TargetMask};
use super::ParseError;
use std::result::Result;
use std::str::FromStr;

mod channel;
mod user;

/// A single target of a message such as PRIVMSG. This can take many different
/// forms:
///
/// ```text
/// msgto      =  channel / ( user [ "%" host ] "@" servername )
/// msgto      =/ ( user "%" host ) / targetmask
/// msgto      =/ nickname / ( nickname "!" user "@" host )
/// channel    =  ( "#" / "+" / ( "!" channelid ) / "&" ) chanstring
///               [ ":" chanstring ]
/// servername =  hostname
/// host       =  hostname / hostaddr
/// hostname   =  shortname *( "." shortname )
/// shortname  =  ( letter / digit ) *( letter / digit / "-" )
///               *( letter / digit )
///                 ; as specified in RFC 1123 [HNAME]
/// hostaddr   =  ip4addr / ip6addr
/// ip4addr    =  1*3digit "." 1*3digit "." 1*3digit "." 1*3digit
/// ip6addr    =  1*hexdigit 7( ":" 1*hexdigit )
/// ip6addr    =/ "0:0:0:0:0:" ( "0" / "FFFF" ) ":" ip4addr
/// nickname   =  ( letter / special ) *8( letter / digit / special / "-" )
/// targetmask =  ( "$" / "#" ) mask
///                 ; see details on allowed masks in section 3.3.1
/// chanstring =  %x01-07 / %x08-09 / %x0B-0C / %x0E-1F / %x21-2B
/// chanstring =/ %x2D-39 / %x3B-FF
///                 ; any octet except NUL, BELL, CR, LF, " ", "," and ":"
/// channelid  = 5( %x41-5A / digit )   ; 5( A-Z / 0-9 )
/// user       =  1*( %x01-09 / %x0B-0C / %x0E-1F / %x21-3F / %x41-FF )
///                 ; any octet except NUL, CR, LF, " " and "@"
/// key        =  1*23( %x01-05 / %x07-08 / %x0C / %x0E-1F / %x21-7F )
///                 ; any 7-bit US_ASCII character,
///                 ; except NUL, CR, LF, FF, h/v TABs, and " "
/// letter     =  %x41-5A / %x61-7A       ; A-Z / a-z
/// digit      =  %x30-39                 ; 0-9
/// hexdigit   =  digit / "A" / "B" / "C" / "D" / "E" / "F"
/// special    =  %x5B-60 / %x7B-7D
///                  ; "[", "]", "\", "`", "_", "^", "{", "|", "}"
/// ```
///
/// This syntax leaves plenty of room for ambiguity, so the following precedence
/// is used by the parser:
///
/// - "#rando.chan" => public channel or server mask? channel
/// - "#*.com" => public channel or server mask? mask
/// - "#user@example.com" => public channel or user@servername? channel
/// - "user%host@example.com" => is the username "user" or "user%host"? "user"
/// - "user%host" => is the username "user%host" or "user"? "user"
/// - "user%host%host" => what is even happening here? invalid, reject
#[derive(PartialEq, Debug)]
pub enum Recipient {
    Channel(Channel),
    Nickname(Nickname),
    NicknameUserHost(Nickname, Username, Host), // nickname!user@host
    TargetMask(TargetMask),
    UserHost(Username, Host),                       // user%host
    UserHostServername(Username, Host, Servername), // user%host@servername
    UserServername(Username, Servername),           // user@servername
}

impl FromStr for Recipient {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.starts_with('#') && raw.contains(&['*', '?'][..]) {
            if let Ok(target_mask) = raw.parse() {
                return Ok(Recipient::TargetMask(target_mask));
            }
        }

        if let Ok(channel) = raw.parse() {
            Ok(Recipient::Channel(channel))
        } else if let Ok(target_mask) = raw.parse() {
            Ok(Recipient::TargetMask(target_mask))
        } else if let Ok(nickname) = raw.parse() {
            Ok(Recipient::Nickname(nickname))
        } else {
            match &raw.matches(&['!', '@', '%'][..]).collect::<String>()[..] {
                "!@" => {
                    let parts: Vec<&str> = raw.split(&['!', '@'][..]).collect();
                    Ok(Recipient::NicknameUserHost(
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                    ))
                }
                "%@" => {
                    let parts: Vec<&str> = raw.split(&['%', '@'][..]).collect();
                    Ok(Recipient::UserHostServername(
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                    ))
                }
                "%" => {
                    let parts: Vec<&str> = raw.split('%').collect();
                    Ok(Recipient::UserHost(parts[0].parse()?, parts[1].parse()?))
                }
                "@" => {
                    let parts: Vec<&str> = raw.split('@').collect();
                    Ok(Recipient::UserServername(
                        parts[0].parse()?,
                        parts[1].parse()?,
                    ))
                }
                _ => Err(ParseError::new("Recipient")),
            }
        }
    }
}

impl From<Recipient> for String {
    fn from(msg_to: Recipient) -> String {
        match msg_to {
            Recipient::Channel(channel) => String::from(channel),
            Recipient::Nickname(nickname) => String::from(nickname),
            Recipient::NicknameUserHost(nickname, user, host) => [
                &String::from(nickname),
                "!",
                &String::from(user),
                "@",
                &String::from(host),
            ]
            .join(""),
            Recipient::TargetMask(target_mask) => String::from(target_mask),
            Recipient::UserHost(user, host) => [String::from(user), String::from(host)].join("%"),
            Recipient::UserHostServername(user, host, servername) => [
                &String::from(user),
                "%",
                &String::from(host),
                "@",
                &String::from(servername),
            ]
            .join(""),
            Recipient::UserServername(user, servername) => {
                [String::from(user), String::from(servername)].join("@")
            }
        }
    }
}

#[cfg(test)]
mod test_recipient {
    use super::super::types::KeywordList;
    use super::*;

    #[test]
    fn valid_list() {
        let keyword_list = KeywordList::<Recipient>::new();
        assert_eq!(Ok(keyword_list), "".parse::<KeywordList<Recipient>>());

        let mut keyword_list = KeywordList::new();
        keyword_list.push(Recipient::Channel("#channel".parse().unwrap()));
        keyword_list.push(Recipient::Nickname("nickname".parse().unwrap()));
        keyword_list.push(Recipient::NicknameUserHost(
            "NUHnickname".parse().unwrap(),
            "NUHuser".parse().unwrap(),
            "NUHhost".parse().unwrap(),
        ));
        keyword_list.push(Recipient::TargetMask("$target.mask".parse().unwrap()));
        keyword_list.push(Recipient::UserHost(
            "UHuser".parse().unwrap(),
            "UHhost".parse().unwrap(),
        ));
        keyword_list.push(Recipient::UserHostServername(
            "UHSuser".parse().unwrap(),
            "UHShost".parse().unwrap(),
            "UHSservername".parse().unwrap(),
        ));
        keyword_list.push(Recipient::UserServername(
            "USuser".parse().unwrap(),
            "USservername".parse().unwrap(),
        ));
        assert_eq!(
            Ok(keyword_list),
            "#channel,nickname,NUHnickname!NUHuser@NUHhost,$target.mask,UHuser%UHhost,UHSuser%UHShost@UHSservername,USuser@USservername".parse::<KeywordList<Recipient>>()
        );
    }

    #[test]
    fn invalid() {
        assert!("".parse::<Recipient>().is_err());
        assert!("user%host%host".parse::<Recipient>().is_err());
        assert!("🥔️".parse::<Recipient>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Recipient::Nickname("mynick".parse().unwrap())),
            "mynick".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::NicknameUserHost(
                "mynick".parse().unwrap(),
                "user".parse().unwrap(),
                "host".parse().unwrap()
            )),
            "mynick!user@host".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::UserServername(
                "user".parse().unwrap(),
                "servername".parse().unwrap()
            )),
            "user@servername".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::Nickname("mynick".parse().unwrap())),
            "mynick".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::Channel("#rando.chan".parse().unwrap())),
            "#rando.chan".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::TargetMask("#*.com".parse().unwrap())),
            "#*.com".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::Channel("#user@example.com".parse().unwrap())),
            "#user@example.com".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::UserHostServername(
                "user".parse().unwrap(),
                "host".parse().unwrap(),
                "example.com".parse().unwrap()
            )),
            "user%host@example.com".parse::<Recipient>()
        );
        assert_eq!(
            Ok(Recipient::UserHost(
                "user".parse().unwrap(),
                "host".parse().unwrap()
            )),
            "user%host".parse::<Recipient>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "#mychan".to_string(),
            String::from(Recipient::Channel("#mychan".parse().unwrap()))
        );
        assert_eq!(
            "mynick".to_string(),
            String::from(Recipient::Nickname("mynick".parse().unwrap()))
        );
        assert_eq!(
            "nickname!user@host".to_string(),
            String::from(Recipient::NicknameUserHost(
                "nickname".parse().unwrap(),
                "user".parse().unwrap(),
                "host".parse().unwrap()
            ))
        );
        assert_eq!(
            "$target.mask".to_string(),
            String::from(Recipient::TargetMask("$target.mask".parse().unwrap()))
        );
        assert_eq!(
            "user%host".to_string(),
            String::from(Recipient::UserHost(
                "user".parse().unwrap(),
                "host".parse().unwrap()
            ))
        );
        assert_eq!(
            "user%host@servername".to_string(),
            String::from(Recipient::UserHostServername(
                "user".parse().unwrap(),
                "host".parse().unwrap(),
                "servername".parse().unwrap()
            ))
        );
        assert_eq!(
            "user@servername".to_string(),
            String::from(Recipient::UserServername(
                "user".parse().unwrap(),
                "servername".parse().unwrap()
            ))
        );

        let mut keyword_list = KeywordList::new();
        keyword_list.push(Recipient::Channel("#channel1".parse().unwrap()));
        keyword_list.push(Recipient::Nickname("nick1".parse().unwrap()));
        keyword_list.push(Recipient::Channel("#channel2".parse().unwrap()));
        keyword_list.push(Recipient::Nickname("nick2".parse().unwrap()));
        assert_eq!(
            "#channel1,nick1,#channel2,nick2".to_string(),
            String::from(keyword_list)
        );
    }
}

#[derive(PartialEq, Debug)]
pub enum Sender {
    User {
        nickname: Nickname,
        user: Option<Username>,
        host: Option<Host>,
    },
    Server(Servername),
}

impl FromStr for Sender {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let (Ok(servername), true) = (raw.parse(), raw.contains('.')) {
            Ok(Sender::Server(servername))
        } else {
            match &raw.matches(&['!', '@'][..]).collect::<String>()[..] {
                "!@" => {
                    let parts: Vec<&str> = raw.split(&['!', '@'][..]).collect();
                    Ok(Sender::User {
                        nickname: parts[0].parse()?,
                        user: Some(parts[1].parse()?),
                        host: Some(parts[2].parse()?),
                    })
                }
                "@" => {
                    let parts: Vec<&str> = raw.split('@').collect();
                    Ok(Sender::User {
                        nickname: parts[0].parse()?,
                        user: None,
                        host: Some(parts[1].parse()?),
                    })
                }
                "" => Ok(Sender::User {
                    nickname: raw.parse()?,
                    user: None,
                    host: None,
                }),
                _ => Err(ParseError::new("Sender")),
            }
        }
    }
}

impl From<Nickname> for Sender {
    fn from(nickname: Nickname) -> Sender {
        Sender::User {
            nickname,
            host: None,
            user: None,
        }
    }
}

impl From<Servername> for Sender {
    fn from(servername: Servername) -> Sender {
        Sender::Server(servername)
    }
}

impl From<Sender> for String {
    fn from(sender: Sender) -> String {
        match sender {
            Sender::Server(servername) => String::from(servername),
            Sender::User {
                nickname,
                user,
                host,
            } => {
                let mut result = String::from(nickname);
                if let Some(host) = host {
                    if let Some(user) = user {
                        result.push('!');
                        result.push_str(&String::from(user))
                    }

                    result.push('@');
                    result.push_str(&String::from(host));
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod test_sender {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<Sender>().is_err());
        assert!("nickname!user".parse::<Sender>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Sender::User {
                nickname: "nickname".parse().unwrap(),
                user: None,
                host: None,
            }),
            "nickname".parse::<Sender>()
        );
        assert_eq!(
            Ok(Sender::User {
                nickname: "nickname".parse().unwrap(),
                user: None,
                host: Some("host.name".parse().unwrap())
            }),
            "nickname@host.name".parse::<Sender>()
        );
        assert_eq!(
            Ok(Sender::User {
                nickname: "nickname".parse().unwrap(),
                user: Some("user".parse().unwrap()),
                host: Some("host.name".parse().unwrap())
            }),
            "nickname!user@host.name".parse::<Sender>()
        );
        assert_eq!(
            Ok(Sender::Server("irc.example.com".parse().unwrap())),
            "irc.example.com".parse::<Sender>()
        );
    }
}
