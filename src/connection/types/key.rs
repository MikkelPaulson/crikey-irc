use super::ParseError;
use std::result::Result;
use std::str::FromStr;

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
pub struct Key(String);

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 1
            || raw.len() > 23
            || !raw.is_ascii()
            || raw.contains(&['\x00', '\x06', '\x09', '\x0a', '\x0b', '\x0d', '\x20'][..])
        {
            Err(ParseError::new("Key"))
        } else {
            Ok(Self(raw.to_string()))
        }
    }
}

impl From<Key> for String {
    fn from(key: Key) -> String {
        key.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_length() {
        assert!("".parse::<Key>().is_err());
        assert!("abcdefghijklmnopqrstuvwx".parse::<Key>().is_err());
    }

    #[test]
    fn invalid_characters() {
        assert!("null\0".parse::<Key>().is_err());
        assert!("carriage\rreturn".parse::<Key>().is_err());
        assert!("line\nfeed".parse::<Key>().is_err());
        assert!(" space".parse::<Key>().is_err());
        assert!("tab\t".parse::<Key>().is_err());
        assert!("vertical\x0btab".parse::<Key>().is_err());
        assert!("acknowledge\x06".parse::<Key>().is_err());
        assert!("potat🥔️".parse::<Key>().is_err());
    }

    #[test]
    fn success() {
        assert_eq!(
            Key("a".to_string()),
            "a".parse::<Key>()
                .expect("1-character string should be accepted.")
        );

        assert_eq!(
            Key("0123456789abcdeABCDE~!#".to_string()),
            "0123456789abcdeABCDE~!#"
                .parse::<Key>()
                .expect("23-character string should be accepted.")
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("a".to_string(), String::from(Key("a".to_string())))
    }
}
