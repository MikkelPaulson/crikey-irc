use super::{ParseError, ServerMask};
use std::result::Result;
use std::str::FromStr;

const LOCAL_PREFIX: char = '&';
const SAFE_PREFIX: char = '!';
const NO_MODE_PREFIX: char = '+';
const PUBLIC_PREFIX: char = '#';

/// A channel is a named group of one or more users which will all receive
/// messages addressed to that channel. According to RFC 2812:
///
/// ```text
/// channel    =  ( "#" / "+" / ( "!" channelid ) / "&" ) chanstring
///               [ ":" chanstring ]
/// chanstring =  %x01-07 / %x08-09 / %x0B-0C / %x0E-1F / %x21-2B
/// chanstring =/ %x2D-39 / %x3B-FF
///                 ; any octet except NUL, BELL, CR, LF, " ", "," and ":"
/// channelid  = 5( %x41-5A / digit )   ; 5( A-Z / 0-9 )
/// ```
///
/// The RFC does a poor job of explaining, but the ':' character precedes a
/// channel mask, that being a server mask to which the channel is restricted.
#[derive(PartialEq, Debug)]
pub struct Channel {
    channel_type: ChannelType,
    channel_name: ChannelName,
    server_mask: Option<ServerMask>,
}

impl FromStr for Channel {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (raw, server_mask) = if let Some(index) = raw.find(':') {
            (&raw[..index], Some(raw[index + 1..].parse()?))
        } else {
            (raw, None)
        };

        if raw.len() < 2 || raw.len() > 50 {
            Err(ParseError::new("Channel"))
        } else {
            if raw.starts_with(SAFE_PREFIX) && raw.len() > 6 && raw.is_char_boundary(6) {
                Ok(Channel {
                    channel_type: ChannelType::Safe(raw[1..6].parse()?),
                    channel_name: raw[6..].parse()?,
                    server_mask,
                })
            } else {
                Ok(Channel {
                    channel_type: match raw.chars().nth(0) {
                        Some(LOCAL_PREFIX) => ChannelType::Local,
                        Some(NO_MODE_PREFIX) => ChannelType::NoMode,
                        Some(PUBLIC_PREFIX) => ChannelType::Public,
                        _ => return Err(ParseError::new("Channel")),
                    },
                    channel_name: raw[1..].parse()?,
                    server_mask,
                })
            }
        }
    }
}

impl From<Channel> for String {
    fn from(channel: Channel) -> String {
        let mut result = String::from(channel.channel_type);
        result.push_str(&String::from(channel.channel_name)[..]);
        if let Some(server_mask) = channel.server_mask {
            result.push(':');
            result.push_str(&String::from(server_mask)[..]);
        }
        result
    }
}

#[cfg(test)]
mod test_channel {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<Channel>().is_err());
        assert!("#".parse::<Channel>().is_err());
        assert!("+".parse::<Channel>().is_err());
        assert!("&".parse::<Channel>().is_err());
        assert!("🥔️".parse::<Channel>().is_err());
        assert!("!ABCDE".parse::<Channel>().is_err());
        assert!("!ABCD🥔️".parse::<Channel>().is_err());
        assert!("noprefix".parse::<Channel>().is_err());
        assert!("#too:many:colons".parse::<Channel>().is_err());
        assert!("&new\nline".parse::<Channel>().is_err());
        assert!("+carriage\rreturn".parse::<Channel>().is_err());
        assert!("!ABCDEnull\0char".parse::<Channel>().is_err());
        assert!("#ding\x07dong".parse::<Channel>().is_err());
        assert!("& space".parse::<Channel>().is_err());
        assert!("+comma,".parse::<Channel>().is_err());
        assert!("!12345:colon".parse::<Channel>().is_err());
        assert!("#01234567890123456789012345678901234567890123456789"
            .parse::<Channel>()
            .is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Channel {
                channel_type: ChannelType::Public,
                channel_name: ChannelName("mypublic".to_string()),
                server_mask: None,
            }),
            "#mypublic".parse::<Channel>()
        );
        assert_eq!(
            Ok(Channel {
                channel_type: ChannelType::Local,
                channel_name: ChannelName("my_local".to_string()),
                server_mask: None,
            }),
            "&my_local".parse::<Channel>()
        );
        assert_eq!(
            Ok(Channel {
                channel_type: ChannelType::NoMode,
                channel_name: ChannelName("my-no-mode".to_string()),
                server_mask: Some("*.example.com".parse().unwrap()),
            }),
            "+my-no-mode:*.example.com".parse::<Channel>()
        );
        assert_eq!(
            Ok(Channel {
                channel_type: ChannelType::Safe(ChannelID("ABC12".to_string())),
                channel_name: ChannelName("3".to_string()),
                server_mask: None,
            }),
            "!ABC123".parse::<Channel>()
        );
        assert_eq!(
            Ok(Channel {
                channel_type: ChannelType::Public,
                channel_name: ChannelName(
                    "0123456789012345678901234567890123456789012345678".to_string()
                ),
                server_mask: None,
            }),
            "#0123456789012345678901234567890123456789012345678".parse::<Channel>()
        );
        assert_eq!(
            Ok(Channel {
                channel_type: ChannelType::Local,
                channel_name: ChannelName("🥔️".to_string()),
                server_mask: None,
            }),
            "&🥔️".parse::<Channel>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "#mychan".to_string(),
            String::from(Channel {
                channel_name: ChannelName("mychan".to_string()),
                channel_type: ChannelType::Public,
                server_mask: None,
            })
        );
        assert_eq!(
            "&localchan".to_string(),
            String::from(Channel {
                channel_name: ChannelName("localchan".to_string()),
                channel_type: ChannelType::Local,
                server_mask: None,
            })
        );
        assert_eq!(
            "+nomode:example.com".to_string(),
            String::from(Channel {
                channel_name: ChannelName("nomode".to_string()),
                channel_type: ChannelType::NoMode,
                server_mask: Some("example.com".parse().unwrap()),
            })
        );
        assert_eq!(
            "!12345safemode".to_string(),
            String::from(Channel {
                channel_name: ChannelName("safemode".to_string()),
                channel_type: ChannelType::Safe(ChannelID("12345".to_string())),
                server_mask: None,
            })
        )
    }
}

/// The type of a channel, also referred to as its namespace.
///
/// - `#` is public, shared among all servers on a network.
/// - `&` is local, not shared with any server
/// - `+` is public but does not support modes such as +o and +v
/// - `!` is public but "safe" and is prefixed with a server-generated channel
///   ID to mitigate name collisions between servers
#[derive(PartialEq, Debug)]
pub enum ChannelType {
    Local,           // Prefix: &
    Safe(ChannelID), // Prefix: ![A-Z0-9]{5}
    NoMode,          // Prefix: +
    Public,          // Prefix: #
}

impl FromStr for ChannelType {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match (raw.len(), raw.chars().nth(0)) {
            (1, Some(LOCAL_PREFIX)) => Ok(ChannelType::Local),
            (1, Some(NO_MODE_PREFIX)) => Ok(ChannelType::NoMode),
            (1, Some(PUBLIC_PREFIX)) => Ok(ChannelType::Public),
            (6, Some(SAFE_PREFIX)) => {
                if let Ok(channel_id) = raw[1..].to_string().parse() {
                    Ok(ChannelType::Safe(channel_id))
                } else {
                    Err(ParseError::new("ChannelType"))
                }
            }
            _ => Err(ParseError::new("ChannelType")),
        }
    }
}

impl From<ChannelType> for String {
    fn from(channel_type: ChannelType) -> String {
        match channel_type {
            ChannelType::Local => LOCAL_PREFIX.to_string(),
            ChannelType::NoMode => NO_MODE_PREFIX.to_string(),
            ChannelType::Public => PUBLIC_PREFIX.to_string(),
            ChannelType::Safe(channel_id) => {
                let mut result = SAFE_PREFIX.to_string();
                result.push_str(&String::from(channel_id)[..]);
                result
            }
        }
    }
}

#[cfg(test)]
mod test_channel_type {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<ChannelType>().is_err());
        assert!("#a".parse::<ChannelType>().is_err());
        assert!("*".parse::<ChannelType>().is_err());
        assert!("🥔️".parse::<ChannelType>().is_err());
        assert!("#ABCDE".parse::<ChannelType>().is_err());
        assert!("!ABCD🥔️".parse::<ChannelType>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(Ok(ChannelType::Public), "#".parse::<ChannelType>());
        assert_eq!(Ok(ChannelType::Local), "&".parse::<ChannelType>());
        assert_eq!(Ok(ChannelType::NoMode), "+".parse::<ChannelType>());
        assert_eq!(
            Ok(ChannelType::Safe(ChannelID("123YZ".to_string()))),
            "!123YZ".parse::<ChannelType>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("#".to_string(), String::from(ChannelType::Public));
        assert_eq!("&".to_string(), String::from(ChannelType::Local));
        assert_eq!("+".to_string(), String::from(ChannelType::NoMode));
        assert_eq!(
            "!12345".to_string(),
            String::from(ChannelType::Safe(ChannelID("12345".to_string())))
        );
    }
}

/// The 5-digit ID attached to a "safe" channel name.
///
/// ```text
/// channelid  = 5( %x41-5A / digit )   ; 5( A-Z / 0-9 )
/// ```
///
/// ```text
/// The channel identifier is a function of the time.  The current time
/// (as defined under UNIX by the number of seconds elapsed since
/// 00:00:00 GMT, January 1, 1970) is converted in a string of five (5)
/// characters using the following base:
/// "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890" (each character has a decimal
/// value starting from 0 for 'A' to 35 for '0').
///
/// The channel identifier therefore has a periodicity of 36^5 seconds
/// (about 700 days).
/// ```
///
/// (Yes, this is backwards from normal base-x encoding.)
#[derive(PartialEq, Debug)]
pub struct ChannelID(String);

impl FromStr for ChannelID {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() != 5 {
            println!("{:?}", raw);
            Err(ParseError::new("ChannelID"))
        } else {
            if raw.contains(|c: char| !c.is_ascii_uppercase() && !c.is_ascii_digit()) {
                Err(ParseError::new("ChannelID"))
            } else {
                Ok(Self(raw.to_string()))
            }
        }
    }
}

impl From<ChannelID> for String {
    fn from(channel_id: ChannelID) -> String {
        channel_id.0
    }
}

#[cfg(test)]
mod test_channel_id {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<ChannelID>().is_err());
        assert!("1234".parse::<ChannelType>().is_err());
        assert!("123456".parse::<ChannelType>().is_err());
        assert!("🥔️".parse::<ChannelType>().is_err());
        assert!("1234🥔️".parse::<ChannelType>().is_err());
        assert!("!ABCDE️".parse::<ChannelType>().is_err());
        assert!("abcde️".parse::<ChannelType>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(ChannelID("12345".to_string())),
            "12345".parse::<ChannelID>()
        );
        assert_eq!(
            Ok(ChannelID("ABXYZ".to_string())),
            "ABXYZ".parse::<ChannelID>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "VWXYZ".to_string(),
            String::from(ChannelID("VWXYZ".to_string())),
        );
    }
}

/// The name/"chanstring" part of a channel identifier.
///
/// ```text
/// chanstring =  %x01-07 / %x08-09 / %x0B-0C / %x0E-1F / %x21-2B
/// chanstring =/ %x2D-39 / %x3B-FF
///                 ; any octet except NUL, BELL, CR, LF, " ", "," and ":"
/// ```
#[derive(PartialEq, Debug)]
pub struct ChannelName(String);

impl FromStr for ChannelName {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 1 || raw.contains(&['\x00', '\x07', '\r', '\n', ' ', ',', ':'][..]) {
            Err(ParseError::new("ChannelName"))
        } else {
            Ok(Self(raw.to_string()))
        }
    }
}

impl From<ChannelName> for String {
    fn from(channel_name: ChannelName) -> String {
        channel_name.0
    }
}

#[cfg(test)]
mod test_channel_name {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<ChannelName>().is_err());
        assert!("new\nline".parse::<ChannelName>().is_err());
        assert!("carriage\rreturn".parse::<ChannelName>().is_err());
        assert!("null\0char".parse::<ChannelName>().is_err());
        assert!("ding\x07dong".parse::<ChannelName>().is_err());
        assert!(" space".parse::<ChannelName>().is_err());
        assert!("comma,".parse::<ChannelName>().is_err());
        assert!("co:on".parse::<ChannelName>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(ChannelName("mychannel".to_string())),
            "mychannel".parse::<ChannelName>()
        );
        assert_eq!(Ok(ChannelName("#".to_string())), "#".parse::<ChannelName>());
        assert_eq!(
            Ok(ChannelName("🥔️".to_string())),
            "🥔️".parse::<ChannelName>()
        );
        assert_eq!(
            Ok(ChannelName("\"".to_string())),
            "\"".parse::<ChannelName>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "mychan".to_string(),
            String::from(ChannelName("mychan".to_string())),
        );
    }
}

/// A password restricting access to a channel. According to RFC 2812:
///
/// ```text
/// key        =  1*23( %x01-05 / %x07-08 / %x0C / %x0E-1F / %x21-7F )
///                 ; any 7-bit US_ASCII character,
///                 ; except NUL, CR, LF, FF, h/v TABs, and " "
/// ```
///
/// Note that the formal notation excludes the ACK character (\x06) rather than
/// FF (\x0c) as the comment indicates. This implementation treats the formal
/// notation as authoritative.
#[derive(PartialEq, Debug)]
pub struct ChannelKey(String);

impl FromStr for ChannelKey {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 1
            || raw.len() > 23
            || !raw.is_ascii()
            || raw.contains(&['\x00', '\x06', '\x09', '\x0a', '\x0b', '\x0d', '\x20'][..])
        {
            Err(ParseError::new("ChannelKey"))
        } else {
            Ok(Self(raw.to_string()))
        }
    }
}

impl From<ChannelKey> for String {
    fn from(key: ChannelKey) -> String {
        key.0
    }
}

#[cfg(test)]
mod test_channel_key {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<ChannelKey>().is_err());
        assert!("abcdefghijklmnopqrstuvwx".parse::<ChannelKey>().is_err());
        assert!("null\0".parse::<ChannelKey>().is_err());
        assert!("carriage\rreturn".parse::<ChannelKey>().is_err());
        assert!("line\nfeed".parse::<ChannelKey>().is_err());
        assert!(" space".parse::<ChannelKey>().is_err());
        assert!("tab\t".parse::<ChannelKey>().is_err());
        assert!("vertical\x0btab".parse::<ChannelKey>().is_err());
        assert!("acknowledge\x06".parse::<ChannelKey>().is_err());
        assert!("potat🥔️".parse::<ChannelKey>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(Ok(ChannelKey("a".to_string())), "a".parse::<ChannelKey>());

        assert_eq!(
            Ok(ChannelKey("0123456789abcdeABCDE~!#".to_string())),
            "0123456789abcdeABCDE~!#".parse::<ChannelKey>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("a".to_string(), String::from(ChannelKey("a".to_string())))
    }
}
