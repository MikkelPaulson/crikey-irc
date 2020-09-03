use super::ParseError;
use std::result::Result;
use std::str::FromStr;

/// The login name of an IRC user (not the nick). According to RFC 2812:
///
/// ```text
/// user       =  1*( %x01-09 / %x0B-0C / %x0E-1F / %x21-3F / %x41-FF )
///                 ; any octet except NUL, CR, LF, " " and "@"
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Username(String);

impl FromStr for Username {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 1 || raw.contains(&['\0', '\r', '\n', ' ', '@'][..]) {
            Err(ParseError::new("Username"))
        } else {
            Ok(Username(raw.to_string()))
        }
    }
}

impl From<Username> for String {
    fn from(username: Username) -> String {
        username.0
    }
}

#[cfg(test)]
mod test_username {
    use super::Username;

    #[test]
    fn invalid() {
        assert!("".parse::<Username>().is_err());
        assert!("null\0".parse::<Username>().is_err());
        assert!("carriage\rreturn".parse::<Username>().is_err());
        assert!("line\nfeed".parse::<Username>().is_err());
        assert!(" space".parse::<Username>().is_err());
        assert!("the@sign".parse::<Username>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(Ok(Username("a".to_string())), "a".parse::<Username>());
        assert_eq!(
            Ok(Username("potat🥔️".to_string())),
            "potat🥔️".parse::<Username>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("a".to_string(), String::from(Username("a".to_string())));
    }
}

/// The nickname by which a user is primarily known. According to RFC 2812:
///
/// ```text
/// nickname   =  ( letter / special ) *8( letter / digit / special / "-" )
/// letter     =  %x41-5A / %x61-7A       ; A-Z / a-z
/// digit      =  %x30-39                 ; 0-9
/// special    =  %x5B-60 / %x7B-7D
///                  ; "[", "]", "\", "`", "_", "^", "{", "|", "}"
/// ```
///
/// Note that this notation limits nicknames to 9 characters, but the RFC
/// elsewhere recommends supporting longer nicknames for forwards compatibility.
/// We currently enforce no upper bound.
#[derive(Clone, PartialEq, Debug)]
pub struct Nickname(String);

impl FromStr for Nickname {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 1 || !raw.is_ascii() {
            Err(ParseError::new("Nickname"))
        } else if let '\x41'..='\x7d' = raw.chars().nth(0).unwrap() {
            if raw[1..].contains(|c: char| {
                if let '\x2d' | '\x30'..='\x39' | '\x41'..='\x7d' = c {
                    false
                } else {
                    true
                }
            }) {
                Err(ParseError::new("Nickname"))
            } else {
                Ok(Self(raw.to_string()))
            }
        } else {
            Err(ParseError::new("Nickname"))
        }
    }
}

impl From<Nickname> for String {
    fn from(nickname: Nickname) -> String {
        nickname.0
    }
}

#[cfg(test)]
mod test_nickname {
    use super::Nickname;

    #[test]
    fn invalid() {
        assert!("".parse::<Nickname>().is_err());
        assert!("potat🥔️".parse::<Nickname>().is_err());
        assert!("2hot4u".parse::<Nickname>().is_err());
        //assert!("nickn@me".parse::<Nickname>().is_err());
        assert!("-minus".parse::<Nickname>().is_err());
        assert!("spaced out".parse::<Nickname>().is_err());
        assert!("new\nline".parse::<Nickname>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(Ok(Nickname("a".to_string())), "a".parse::<Nickname>());
        assert_eq!(
            Ok(Nickname("n-name".to_string())),
            "n-name".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("I2hot4u".to_string())),
            "I2hot4u".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("ABCDEFGHI".to_string())),
            "ABCDEFGHI".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("JKLMNOPQR".to_string())),
            "JKLMNOPQR".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("STUVWXYZ0".to_string())),
            "STUVWXYZ0".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("abcdefghi".to_string())),
            "abcdefghi".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("jklmnopqr".to_string())),
            "jklmnopqr".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("stuvwxyz1".to_string())),
            "stuvwxyz1".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("x23456789".to_string())),
            "x23456789".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("[]\\`_^{|}".to_string())),
            "[]\\`_^{|}".parse::<Nickname>()
        );
        assert_eq!(
            Ok(Nickname("abcdefghijklmnopqrstuvwxyz".to_string())),
            "abcdefghijklmnopqrstuvwxyz".parse::<Nickname>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("a".to_string(), String::from(Nickname("a".to_string())));
    }
}
