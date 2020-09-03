use super::ParseError;
use std::net::IpAddr;
use std::result::Result;
use std::str::FromStr;

/// A hostname or IP address.
#[derive(PartialEq, Debug)]
pub enum Host {
    Hostaddr(IpAddr),
    Hostname(Hostname),
}

impl FromStr for Host {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(ipaddr) = raw.parse() {
            Ok(Host::Hostaddr(ipaddr))
        } else if let Ok(hostname) = raw.parse() {
            Ok(Host::Hostname(hostname))
        } else {
            Err(ParseError::new("Host"))
        }
    }
}

impl From<Host> for String {
    fn from(host: Host) -> String {
        match host {
            Host::Hostaddr(ip_addr) => ip_addr.to_string(),
            Host::Hostname(hostname) => String::from(hostname),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Servername(String);

impl FromStr for Servername {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        name_from_string(raw)
            .map(|s| Self(s))
            .ok_or(ParseError::new("Servername"))
    }
}

impl From<Servername> for String {
    fn from(servername: Servername) -> String {
        servername.0
    }
}

#[derive(PartialEq, Debug)]
pub struct Hostname(String);

impl FromStr for Hostname {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        name_from_string(raw)
            .map(|s| Self(s))
            .ok_or(ParseError::new("Hostname"))
    }
}

impl From<Hostname> for String {
    fn from(hostname: Hostname) -> String {
        hostname.0
    }
}

fn name_from_string(raw: &str) -> Option<String> {
    for raw_part in raw.split('.') {
        if raw_part.len() < 1
            || !raw_part.starts_with(|c: char| c.is_ascii_alphanumeric())
            || !raw_part.ends_with(|c: char| c.is_ascii_alphanumeric())
            || raw_part.contains(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
        {
            return None;
        }
    }

    Some(raw.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_chars() {
        assert!("abc.d\nf.ghi".parse::<Host>().is_err());
        assert!("abc.d🥔️f.ghi".parse::<Host>().is_err());
        assert!("abc.d f.ghi".parse::<Host>().is_err());
        assert!("abc.-ef.ghi".parse::<Host>().is_err());
        assert!("abc.de-.ghi".parse::<Host>().is_err());
        assert!("".parse::<Host>().is_err());
    }

    #[test]
    fn invalid_dot_placement() {
        assert!("abc..ghi".parse::<Host>().is_err());
        assert!(".a".parse::<Host>().is_err());

        // All non-relative hostnames technically terminate with a trailing dot,
        // but the RFC disagrees.
        assert!("abc.def.ghi.".parse::<Host>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Host::Hostname(Hostname("abcdefghi".to_string())),
            "abcdefghi"
                .parse::<Host>()
                .expect("Hostname without dots should be valid")
        );
        assert_eq!(
            Host::Hostname(Hostname("abc.d-f.ghi".to_string())),
            "abc.d-f.ghi"
                .parse::<Host>()
                .expect("Dotted hostname should be valid")
        );
    }

    #[test]
    fn ipv4() {
        assert_eq!(
            Host::Hostaddr("1.2.3.4".parse().unwrap()),
            "1.2.3.4"
                .parse::<Host>()
                .expect("Expect host mask to accept an IPv4 address")
        )
    }

    #[test]
    fn ipv6() {
        assert_eq!(
            Host::Hostaddr("::1".parse().unwrap()),
            "::1"
                .parse::<Host>()
                .expect("Expect host mask to accept an IPv6 address")
        );
        assert_eq!(
            Host::Hostaddr("2001:db8::ff00:42:8329".parse().unwrap()),
            "2001:db8::ff00:42:8329"
                .parse::<Host>()
                .expect("Expect host mask to accept an IPv6 address")
        );
        assert_eq!(
            Host::Hostaddr("2001:0db8:0000:0000:0000:ff00:0042:8329".parse().unwrap()),
            "2001:0db8:0000:0000:0000:ff00:0042:8329"
                .parse::<Host>()
                .expect("Expect host mask to accept an IPv6 address")
        );
    }

    #[test]
    fn valid_server_and_host_name() {
        assert_eq!(
            Servername("abc.def.ghi".to_string()),
            "abc.def.ghi"
                .parse::<Servername>()
                .expect("Failed to accept a valid server name")
        );
        assert_eq!(
            Hostname("abc.def.ghi".to_string()),
            "abc.def.ghi"
                .parse::<Hostname>()
                .expect("Failed to accept a valid hostname")
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "1.2.3.4".to_string(),
            String::from(Host::Hostaddr("1.2.3.4".parse::<IpAddr>().unwrap()))
        );
        assert_eq!(
            "abc.def.ghi".to_string(),
            String::from(Host::Hostname(Hostname("abc.def.ghi".to_string())))
        );
        assert_eq!(
            "abc.def.ghi".to_string(),
            String::from(Hostname("abc.def.ghi".to_string()))
        );
        assert_eq!(
            "abc.def.ghi".to_string(),
            String::from(Servername("abc.def.ghi".to_string()))
        );
    }
}
