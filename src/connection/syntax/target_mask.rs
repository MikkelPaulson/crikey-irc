use super::ParseError;
use std::result::Result;
use std::str::FromStr;

const HOST_PREFIX: char = '#';
const SERVER_PREFIX: char = '$';

/// A mask used to reference multiple servers or hosts. According to RFC 2812:
///
/// ```text
/// targetmask =  ( "$" / "#" ) mask
///                 ; see details on allowed masks in section 3.3.1
/// servername =  hostname
/// hostname   =  shortname *( "." shortname )
/// shortname  =  ( letter / digit ) *( letter / digit / "-" )
///               *( letter / digit )
///                 ; as specified in RFC 1123 [HNAME]
/// letter     =  %x41-5A / %x61-7A       ; A-Z / a-z
/// digit      =  %x30-39                 ; 0-9
/// ```
///
/// ```text
/// The <msgtarget> parameter may also be a host mask (#<mask>) or server
/// mask ($<mask>).  In both cases the server will only send the PRIVMSG
/// to those who have a server or host matching the mask.  The mask MUST
/// have at least 1 (one) "." in it and no wildcards following the last
/// ".".  This requirement exists to prevent people sending messages to
/// "#*" or "$*", which would broadcast to all users.  Wildcards are the
/// '*' and '?'  characters.  This extension to the PRIVMSG command is
/// only available to operators.
/// ```
///
/// It's worth noting that the syntax listed implicitly covers IPv4 addresses but
/// not IPv6. This is a faithful implementation of the standard.
#[derive(PartialEq, Debug)]
pub enum TargetMask {
    Host(HostMask),     // #xyz
    Server(ServerMask), // $xyz
}

impl FromStr for TargetMask {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.chars().nth(0) {
            Some(HOST_PREFIX) => Ok(TargetMask::Host(raw[1..].parse()?)),
            Some(SERVER_PREFIX) => Ok(TargetMask::Server(raw[1..].parse()?)),
            _ => Err(ParseError::new("TargetMask")),
        }
    }
}

impl From<TargetMask> for String {
    fn from(target_mask: TargetMask) -> String {
        match target_mask {
            TargetMask::Host(host_mask) => {
                let mut result = HOST_PREFIX.to_string();
                result.push_str(&String::from(host_mask)[..]);
                result
            }
            TargetMask::Server(server_mask) => {
                let mut result = SERVER_PREFIX.to_string();
                result.push_str(&String::from(server_mask)[..]);
                result
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct HostMask(String);

impl FromStr for HostMask {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        mask_from_string(raw)
            .map(|s| Self(s))
            .ok_or(ParseError::new("HostMask"))
    }
}

impl From<HostMask> for String {
    fn from(host_mask: HostMask) -> String {
        host_mask.0
    }
}

#[derive(PartialEq, Debug)]
pub struct ServerMask(String);

impl FromStr for ServerMask {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        mask_from_string(raw)
            .map(|s| Self(s))
            .ok_or(ParseError::new("ServerMask"))
    }
}

impl From<ServerMask> for String {
    fn from(server_mask: ServerMask) -> String {
        server_mask.0
    }
}

fn mask_from_string(raw: &str) -> Option<String> {
    if raw.len() > 2
        && raw.is_ascii()
        && raw.contains('.')
        && !raw.split('.').last()?.contains(&['*', '?'][..])
    {
        for raw_part in raw.split('.') {
            if raw_part.len() < 1
                || !raw_part
                    .starts_with(|c: char| c.is_ascii_alphanumeric() || c == '*' || c == '?')
                || !raw_part.ends_with(|c: char| c.is_ascii_alphanumeric() || c == '*' || c == '?')
                || raw_part.contains(|c: char| {
                    !c.is_ascii_alphanumeric() && c != '*' && c != '?' && c != '-'
                })
            {
                return None;
            }
        }

        Some(raw.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_chars() {
        assert!("#abc.d\nf.ghi".parse::<TargetMask>().is_err());
        assert!("$abc.d🥔️f.ghi".parse::<TargetMask>().is_err());
        assert!("#abc.d f.ghi".parse::<TargetMask>().is_err());
        assert!("$abc.-ef.ghi".parse::<TargetMask>().is_err());
        assert!("#abc.de-.ghi".parse::<TargetMask>().is_err());
        assert!("$".parse::<TargetMask>().is_err());
    }

    #[test]
    fn invalid_prefix() {
        assert!("!abc.def.ghi".parse::<TargetMask>().is_err());
        assert!("abc.def.ghi".parse::<TargetMask>().is_err());
    }

    #[test]
    fn wildcard_in_last_group() {
        assert!("#abc.def.g*i".parse::<TargetMask>().is_err());
    }

    #[test]
    fn invalid_dot_placement() {
        assert!("#abcdefghi".parse::<TargetMask>().is_err());
        assert!("#abc..ghi".parse::<TargetMask>().is_err());
        assert!("#.a".parse::<TargetMask>().is_err());

        // All non-relative hostnames technically terminate with a trailing dot,
        // but the RFC disagrees.
        assert!("#abc.def.ghi.".parse::<TargetMask>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            TargetMask::Host(HostMask("abc.def.ghi".to_string())),
            "#abc.def.ghi"
                .parse::<TargetMask>()
                .expect("Explicit host mask should be valid.")
        );
        assert_eq!(
            TargetMask::Server(ServerMask("a-c.d-f.g-i".to_string())),
            "$a-c.d-f.g-i"
                .parse::<TargetMask>()
                .expect("Explicit server mask should be valid.")
        );
        assert_eq!(
            TargetMask::Host(HostMask("abc.*.ghi".to_string())),
            "#abc.*.ghi"
                .parse::<TargetMask>()
                .expect("Host mask with wildcard should be valid.")
        );
        assert_eq!(
            TargetMask::Server(ServerMask("???.*.ghi".to_string())),
            "$???.*.ghi"
                .parse::<TargetMask>()
                .expect("Server mask with multiple wildcards should be valid.")
        );
    }

    #[test]
    fn ipv4() {
        assert_eq!(
            TargetMask::Host(HostMask("1.2.3.4".to_string())),
            "#1.2.3.4"
                .parse::<TargetMask>()
                .expect("Expect host mask to accept an IPv4 address.")
        )
    }

    #[test]
    fn ipv6() {
        // Sadly, IPv6 isn't supported according to the RFC.
        assert!("$::1".parse::<TargetMask>().is_err());
        assert!("#2001:db8::ff00:42:8329".parse::<TargetMask>().is_err());
        assert!("$2001:0db8:0000:0000:0000:ff00:0042:8329"
            .parse::<TargetMask>()
            .is_err());
    }

    #[test]
    fn invalid_server_and_host_mask() {
        assert!("#abc.def.ghi".parse::<HostMask>().is_err());
        assert!("$a-c.d-f.g-i".parse::<ServerMask>().is_err());
    }

    #[test]
    fn valid_server_and_host_mask() {
        assert_eq!(
            ServerMask("abc.def.ghi".to_string()),
            "abc.def.ghi"
                .parse::<ServerMask>()
                .expect("Expect server mask to accept a valid hostname.")
        );
        assert_eq!(
            HostMask("abc.*.ghi".to_string()),
            "abc.*.ghi"
                .parse::<HostMask>()
                .expect("Expect host mask to accept a valid hostname.")
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "$abc.def.ghi".to_string(),
            String::from(TargetMask::Server(ServerMask("abc.def.ghi".to_string())))
        );
        assert_eq!(
            "#abc.def.ghi".to_string(),
            String::from(TargetMask::Host(HostMask("abc.def.ghi".to_string())))
        );
        assert_eq!(
            "abc.def.ghi".to_string(),
            String::from(HostMask("abc.def.ghi".to_string()))
        );
        assert_eq!(
            "abc.def.ghi".to_string(),
            String::from(ServerMask("abc.def.ghi".to_string()))
        );
    }
}
