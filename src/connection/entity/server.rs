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

impl From<IpAddr> for Host {
    fn from(ip_addr: IpAddr) -> Host {
        Host::Hostaddr(ip_addr)
    }
}

impl From<Hostname> for Host {
    fn from(hostname: Hostname) -> Host {
        Host::Hostname(hostname)
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

#[cfg(test)]
mod test_host {
    use super::*;

    #[test]
    fn invalid() {
        assert!("abc.d\nf.ghi".parse::<Host>().is_err());
        assert!("abc.d🥔️f.ghi".parse::<Host>().is_err());
        assert!("abc.d f.ghi".parse::<Host>().is_err());
        assert!("abc.-ef.ghi".parse::<Host>().is_err());
        assert!("abc.de-.ghi".parse::<Host>().is_err());
        assert!("".parse::<Host>().is_err());
        assert!("abc..ghi".parse::<Host>().is_err());
        assert!(".a".parse::<Host>().is_err());
        assert!("abc.def.ghi.".parse::<Host>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Host::Hostname(Hostname("abcdefghi".to_string()))),
            "abcdefghi".parse::<Host>()
        );
        assert_eq!(
            Ok(Host::Hostname(Hostname("abc.d-f.ghi".to_string()))),
            "abc.d-f.ghi".parse::<Host>()
        );
        assert_eq!(
            Ok(Host::Hostaddr("1.2.3.4".parse().unwrap())),
            "1.2.3.4".parse::<Host>()
        );
        assert_eq!(
            Ok(Host::Hostaddr("::1".parse().unwrap())),
            "::1".parse::<Host>()
        );
    }

    #[test]
    fn from_types() {
        assert_eq!(
            Host::Hostaddr("127.0.0.1".parse().unwrap()),
            Host::from("127.0.0.1".parse::<IpAddr>().unwrap())
        );
        assert_eq!(
            Host::Hostname(Hostname("example.com".to_string())),
            Host::from("example.com".parse::<Hostname>().unwrap())
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "1.2.3.4".to_string(),
            String::from(Host::Hostaddr("1.2.3.4".parse().unwrap()))
        );
        assert_eq!(
            "abc.def.ghi".to_string(),
            String::from(Host::Hostname(Hostname("abc.def.ghi".to_string())))
        );
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
mod test_hostname_servername {
    use super::*;

    #[test]
    fn invalid() {
        assert!("abc.d\nf.ghi".parse::<Hostname>().is_err());
        assert!("abc.d🥔️f.ghi".parse::<Servername>().is_err());
        assert!("abc.d f.ghi".parse::<Hostname>().is_err());
        assert!("abc.-ef.ghi".parse::<Servername>().is_err());
        assert!("abc.de-.ghi".parse::<Hostname>().is_err());
        assert!("".parse::<Servername>().is_err());
        assert!("abc..ghi".parse::<Hostname>().is_err());
        assert!(".a".parse::<Servername>().is_err());
        assert!("abc.def.ghi.".parse::<Hostname>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Servername("abc.def.ghi".to_string())),
            "abc.def.ghi".parse::<Servername>()
        );
        assert_eq!(
            Ok(Hostname("abc.def.ghi".to_string())),
            "abc.def.ghi".parse::<Hostname>()
        );
    }

    #[test]
    fn into_string() {
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
