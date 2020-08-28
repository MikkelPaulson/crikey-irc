use super::ParseError;
use std::result::Result;
use std::str::FromStr;

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
#[derive(PartialEq, Debug)]
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
mod tests {
    use super::Nickname;

    #[test]
    fn too_short() {
        assert!("".parse::<Nickname>().is_err());
    }

    #[test]
    fn not_ascii() {
        assert!("potat🥔️".parse::<Nickname>().is_err());
    }

    #[test]
    fn invalid_chars() {
        assert!("2hot4u".parse::<Nickname>().is_err());
        //assert!("nickn@me".parse::<Nickname>().is_err());
        assert!("-minus".parse::<Nickname>().is_err());
        assert!("spaced out".parse::<Nickname>().is_err());
        assert!("new\nline".parse::<Nickname>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Nickname("a".to_string()),
            "a".parse::<Nickname>()
                .expect("Single-character nickname should be accepted.")
        );
        assert_eq!(
            Nickname("n-name".to_string()),
            "n-name"
                .parse::<Nickname>()
                .expect("Hyphen after the first character should be accepted.")
        );
        assert_eq!(
            Nickname("I2hot4u".to_string()),
            "I2hot4u"
                .parse::<Nickname>()
                .expect("Numerals after first character should be accepted.")
        );
        assert_eq!(
            Nickname("ABCDEFGHI".to_string()),
            "ABCDEFGHI"
                .parse::<Nickname>()
                .expect("Uppercase characters should be accepted.")
        );
        assert_eq!(
            Nickname("JKLMNOPQR".to_string()),
            "JKLMNOPQR"
                .parse::<Nickname>()
                .expect("Uppercase characters should be accepted.")
        );
        assert_eq!(
            Nickname("STUVWXYZ0".to_string()),
            "STUVWXYZ0"
                .parse::<Nickname>()
                .expect("Uppercase characters and numerals should be accepted.")
        );
        assert_eq!(
            Nickname("abcdefghi".to_string()),
            "abcdefghi"
                .parse::<Nickname>()
                .expect("Lowercase characters should be accepted.")
        );
        assert_eq!(
            Nickname("jklmnopqr".to_string()),
            "jklmnopqr"
                .parse::<Nickname>()
                .expect("Lowercase characters should be accepted.")
        );
        assert_eq!(
            Nickname("stuvwxyz1".to_string()),
            "stuvwxyz1"
                .parse::<Nickname>()
                .expect("Lowercase characters and numerals should be accepted.")
        );
        assert_eq!(
            Nickname("x23456789".to_string()),
            "x23456789"
                .parse::<Nickname>()
                .expect("Numerals should be accepted.")
        );
        assert_eq!(
            Nickname("[]\\`_^{|}".to_string()),
            "[]\\`_^{|}"
                .parse::<Nickname>()
                .expect("Special characters should be accepted.")
        );
        assert_eq!(
            Nickname("abcdefghijklmnopqrstuvwxyz".to_string()),
            "abcdefghijklmnopqrstuvwxyz"
                .parse::<Nickname>()
                .expect("Strings longer than 9 characters should be accepted.")
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("a".to_string(), String::from(Nickname("a".to_string())));
    }
}
