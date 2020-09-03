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
pub struct User(String);

impl FromStr for User {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 1 || raw.contains(&['\0', '\r', '\n', ' ', '@'][..]) {
            Err(ParseError::new("User"))
        } else {
            Ok(User(raw.to_string()))
        }
    }
}

impl From<User> for String {
    fn from(user: User) -> String {
        user.0
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn too_short() {
        assert!("".parse::<User>().is_err());
    }

    #[test]
    fn invalid_chars() {
        assert!("null\0".parse::<User>().is_err());
        assert!("carriage\rreturn".parse::<User>().is_err());
        assert!("line\nfeed".parse::<User>().is_err());
        assert!(" space".parse::<User>().is_err());
        assert!("the@sign".parse::<User>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            User("a".to_string()),
            "a".parse::<User>()
                .expect("1-character string should be accepted.")
        );
        assert_eq!(
            User("potat🥔️".to_string()),
            "potat🥔️"
                .parse::<User>()
                .expect("UTF-8 string should be accepted.")
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("a".to_string(), String::from(User("a".to_string())));
    }
}
