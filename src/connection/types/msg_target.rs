use super::{Channel, Host, Nickname, ParseError, Servername, TargetMask, User};
use std::borrow::Borrow;
use std::result::Result;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct MsgTarget(Vec<MsgTo>);

impl FromStr for MsgTarget {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut targets = Vec::<MsgTo>::new();
        for msg_to in raw.split(',') {
            targets.push(msg_to.parse()?);
        }
        Ok(Self(targets))
    }
}

impl From<MsgTarget> for String {
    fn from(msg_target: MsgTarget) -> String {
        msg_target
            .0
            .into_iter()
            .map(|a| String::from(a))
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Borrow<Vec<MsgTo>> for MsgTarget {
    fn borrow(&self) -> &Vec<MsgTo> {
        &self.0
    }
}

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
pub enum MsgTo {
    Channel(Channel),
    Nickname(Nickname),
    NicknameUserHost(Nickname, User, Host), // nickname!user@host
    TargetMask(TargetMask),
    UserHost(User, Host),                       // user%host
    UserHostServername(User, Host, Servername), // user%host@servername
    UserServername(User, Servername),           // user@servername
}

impl FromStr for MsgTo {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.starts_with('#') && raw.contains(&['*', '?'][..]) {
            if let Ok(target_mask) = raw.parse() {
                return Ok(MsgTo::TargetMask(target_mask));
            }
        }

        if let Ok(channel) = raw.parse() {
            Ok(MsgTo::Channel(channel))
        } else if let Ok(target_mask) = raw.parse() {
            Ok(MsgTo::TargetMask(target_mask))
        } else if let Ok(nickname) = raw.parse() {
            Ok(MsgTo::Nickname(nickname))
        } else {
            match &raw.matches(&['!', '@', '%'][..]).collect::<String>()[..] {
                "!@" => {
                    let parts: Vec<&str> = raw.split(&['!', '@'][..]).collect();
                    Ok(MsgTo::NicknameUserHost(
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                    ))
                }
                "%@" => {
                    let parts: Vec<&str> = raw.split(&['%', '@'][..]).collect();
                    Ok(MsgTo::UserHostServername(
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                    ))
                }
                "%" => {
                    let parts: Vec<&str> = raw.split('%').collect();
                    Ok(MsgTo::UserHost(parts[0].parse()?, parts[1].parse()?))
                }
                "@" => {
                    let parts: Vec<&str> = raw.split('@').collect();
                    Ok(MsgTo::UserServername(parts[0].parse()?, parts[1].parse()?))
                }
                _ => Err(ParseError::new("MsgTo")),
            }
        }
    }
}

impl From<MsgTo> for String {
    fn from(msg_to: MsgTo) -> String {
        match msg_to {
            MsgTo::Channel(channel) => String::from(channel),
            MsgTo::Nickname(nickname) => String::from(nickname),
            MsgTo::NicknameUserHost(nickname, user, host) => [
                &String::from(nickname),
                "!",
                &String::from(user),
                "@",
                &String::from(host),
            ]
            .join(""),
            MsgTo::TargetMask(target_mask) => String::from(target_mask),
            MsgTo::UserHost(user, host) => [String::from(user), String::from(host)].join("%"),
            MsgTo::UserHostServername(user, host, servername) => [
                &String::from(user),
                "%",
                &String::from(host),
                "@",
                &String::from(servername),
            ]
            .join(""),
            MsgTo::UserServername(user, servername) => {
                [String::from(user), String::from(servername)].join("@")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_msgtarget() {
        assert!("".parse::<MsgTarget>().is_err());
    }

    #[test]
    fn valid_msgtarget() {
        assert_eq!(
            MsgTarget(vec![MsgTo::Nickname("mynick".parse().unwrap())]),
            "mynick".parse::<MsgTarget>().unwrap()
        );
        assert_eq!(
            MsgTarget(vec![MsgTo::Channel("#mychan".parse().unwrap())]),
            "#mychan".parse::<MsgTarget>().unwrap()
        );
        assert_eq!(
            MsgTarget(vec![MsgTo::TargetMask("$my.target.mask".parse().unwrap())]),
            "$my.target.mask".parse::<MsgTarget>().unwrap()
        );
        assert_eq!(
            MsgTarget(vec![
                MsgTo::Channel("#channel".parse().unwrap()),
                MsgTo::Nickname("nickname".parse().unwrap()),
                MsgTo::NicknameUserHost("NUHnickname".parse().unwrap(), "NUHuser".parse().unwrap(), "NUHhost".parse().unwrap()),
                MsgTo::TargetMask("$target.mask".parse().unwrap()),
                MsgTo::UserHost("UHuser".parse().unwrap(), "UHhost".parse().unwrap()),
                MsgTo::UserHostServername("UHSuser".parse().unwrap(), "UHShost".parse().unwrap(), "UHSservername".parse().unwrap()),
                MsgTo::UserServername("USuser".parse().unwrap(), "USservername".parse().unwrap()),
            ]),
            "#channel,nickname,NUHnickname!NUHuser@NUHhost,$target.mask,UHuser%UHhost,UHSuser%UHShost@UHSservername,USuser@USservername".parse::<MsgTarget>().unwrap()
        );
    }

    #[test]
    fn invalid_msgto() {
        assert!("".parse::<MsgTo>().is_err());
        assert!("user%host%host".parse::<MsgTo>().is_err());
        assert!("🥔️".parse::<MsgTo>().is_err());
    }

    #[test]
    fn valid_msgto() {
        assert_eq!(
            MsgTo::Nickname("mynick".parse().unwrap()),
            "mynick".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::NicknameUserHost(
                "mynick".parse().unwrap(),
                "user".parse().unwrap(),
                "host".parse().unwrap()
            ),
            "mynick!user@host".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::UserServername("user".parse().unwrap(), "servername".parse().unwrap()),
            "user@servername".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::Nickname("mynick".parse().unwrap()),
            "mynick".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::Channel("#rando.chan".parse().unwrap()),
            "#rando.chan".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::TargetMask("#*.com".parse().unwrap()),
            "#*.com".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::Channel("#user@example.com".parse().unwrap()),
            "#user@example.com".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::UserHostServername(
                "user".parse().unwrap(),
                "host".parse().unwrap(),
                "example.com".parse().unwrap()
            ),
            "user%host@example.com".parse::<MsgTo>().unwrap()
        );
        assert_eq!(
            MsgTo::UserHost("user".parse().unwrap(), "host".parse().unwrap()),
            "user%host".parse::<MsgTo>().unwrap()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "#mychan".to_string(),
            String::from(MsgTo::Channel("#mychan".parse().unwrap()))
        );
        assert_eq!(
            "mynick".to_string(),
            String::from(MsgTo::Nickname("mynick".parse().unwrap()))
        );
        assert_eq!(
            "nickname!user@host".to_string(),
            String::from(MsgTo::NicknameUserHost(
                "nickname".parse().unwrap(),
                "user".parse().unwrap(),
                "host".parse().unwrap()
            ))
        );
        assert_eq!(
            "$target.mask".to_string(),
            String::from(MsgTo::TargetMask("$target.mask".parse().unwrap()))
        );
        assert_eq!(
            "user%host".to_string(),
            String::from(MsgTo::UserHost(
                "user".parse().unwrap(),
                "host".parse().unwrap()
            ))
        );
        assert_eq!(
            "user%host@servername".to_string(),
            String::from(MsgTo::UserHostServername(
                "user".parse().unwrap(),
                "host".parse().unwrap(),
                "servername".parse().unwrap()
            ))
        );
        assert_eq!(
            "user@servername".to_string(),
            String::from(MsgTo::UserServername(
                "user".parse().unwrap(),
                "servername".parse().unwrap()
            ))
        );

        assert_eq!(
            "#channel1,nick1,#channel2,nick2".to_string(),
            String::from(MsgTarget(vec![
                MsgTo::Channel("#channel1".parse().unwrap()),
                MsgTo::Nickname("nick1".parse().unwrap()),
                MsgTo::Channel("#channel2".parse().unwrap()),
                MsgTo::Nickname("nick2".parse().unwrap()),
            ]))
        );
    }
}
