use super::super::types::{
    Channel, ChannelKey, KeywordList, Nickname, ParseError, Recipient, Sender, ServerMask,
    Servername, StatsQuery, TargetMask, User,
};
use super::MessageParams;
use std::result::Result;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Command {
    // Connection registration
    Pass {
        password: String,
    },
    Nick {
        nickname: Nickname,
    },
    User {
        username: User,
        mode: u8,
        realname: String,
    },
    Oper {
        user: User,
        password: String,
    },
    UserMode {
        nickname: Nickname,
        modes: String,
    },
    Service {
        nickname: Nickname,
        distribution: TargetMask,
        info: String,
    },
    Quit {
        message: Option<String>,
    },
    SQuit {
        server: Servername,
        comment: String,
    },

    // Channel operations
    Join {
        channels: KeywordList<Channel>,
        keys: KeywordList<ChannelKey>,
    },
    Part {
        channels: KeywordList<Channel>,
        message: Option<String>,
    },
    ChannelMode {
        channel: Channel,
        modes: String,
    },
    Topic {
        channel: Channel,
        topic: Option<String>,
    },
    Names {
        channels: KeywordList<Channel>,
        target: Option<Servername>,
    },
    List {
        channels: KeywordList<Channel>,
        target: Option<Servername>,
    },
    Invite {
        nickname: Nickname,
        channel: Channel,
    },
    Kick {
        channels: KeywordList<Channel>,
        users: KeywordList<User>,
        comment: Option<String>,
    },

    // Sending messages
    Privmsg {
        recipients: KeywordList<Recipient>,
        message: String,
    },
    Notice {
        recipients: KeywordList<Recipient>,
        message: String,
    },

    // Server queries and commands
    Motd {
        target: Option<Servername>,
    },
    LUsers {
        mask: Option<ServerMask>,
        target: Option<Servername>,
    },
    Version {
        target: Option<Servername>,
    },
    Stats {
        query: Option<StatsQuery>,
        target: Option<ServerMask>,
    },
    Links {
        mask: Option<ServerMask>,
        target: Option<Servername>,
    },
    Time {
        target: Option<Servername>,
    },
    Connect {
        target: Servername,
        port: u16,
        remote: Option<Servername>,
    },
    Trace {
        target: Option<Servername>,
    }, // TODO: add nickname
    Admin {
        target: Option<Servername>,
    }, // TODO: add nickname
    Info {
        target: Option<Servername>,
    }, // TODO: add nickname

    // Service query and commands
    ServList {
        mask: Option<String>,
        service_type: Option<String>,
    },
    SQuery {
        recipient: Recipient,
        message: String,
    },

    // User based queries
    Who {
        mask: String,
        op_only: bool,
    },
    WhoIs {
        mask: String,
        target: Option<Servername>,
    },
    WhoWas {
        targets: KeywordList<Nickname>,
        count: Option<u16>,
        target: Option<ServerMask>,
    },

    // Miscellaneous messages
    Kill {
        nickname: Nickname,
        comment: String,
    },
    Ping {
        from: Option<Sender>,
        to: Option<Sender>,
    },
    Pong {
        from: Sender,
        to: Option<Sender>,
    },
    Error {
        message: String,
    },

    // Optional features
    Away {
        message: Option<String>,
    },
    Rehash,
    Die,
    Restart,
    Summon {
        user: User,
        target: Option<Servername>,
        channel: Option<Channel>,
    },
    Users {
        target: Option<Servername>,
    },
    WallOps {
        message: String,
    },
    UserHost {
        nicknames: KeywordList<Nickname>,
    },
    IsOn {
        nicknames: KeywordList<Nickname>,
    },
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        // Message handles the sender prefix, so we should start with the command
        let (raw_command, raw_args) = if let Some(index) = raw.find(' ') {
            (&raw[..index], &raw[index + 1..])
        } else {
            (raw, "")
        };

        let args = raw_args.parse::<MessageParams>()?;

        match (raw_command, args.len()) {
            ("PASS", 1) => Ok(Command::Pass {
                password: args[0].to_owned(),
            }),
            ("NICK", 1) => Ok(Command::Nick {
                nickname: args[0].parse()?,
            }),
            ("USER", 4) => Ok(Command::User {
                username: args[0].parse()?,
                mode: args[1].parse().map_err(|_| ParseError::new("Command"))?,
                realname: args[3].to_owned(),
            }),
            ("OPER", 2) => Ok(Command::Oper {
                user: args[0].parse()?,
                password: args[1].to_owned(),
            }),
            ("MODE", 2..=15) => {
                let (_, modes) =
                    raw_args.split_at(raw_args.find(' ').map(|i| i + 1).unwrap_or(raw_args.len()));
                if let Ok(channel) = args[0].parse() {
                    Ok(Command::ChannelMode {
                        channel,
                        modes: modes.to_string(),
                    })
                } else if let Ok(nickname) = args[0].parse() {
                    Ok(Command::UserMode {
                        nickname,
                        modes: modes.to_string(),
                    })
                } else {
                    Err(ParseError::new("Command"))
                }
            }
            ("SERVICE", 6) => Ok(Command::Service {
                nickname: args[0].parse()?,
                distribution: args[2].parse()?,
                info: args[4].to_owned(),
            }),
            ("QUIT", 0) => Ok(Command::Quit { message: None }),
            ("QUIT", 1) => Ok(Command::Quit {
                message: Some(args[0].to_string()),
            }),
            ("SQUIT", 2) => Ok(Command::SQuit {
                server: args[0].parse()?,
                comment: args[1].to_string(),
            }),
            ("JOIN", 1) => {
                if args[0] == "0" {
                    Ok(Command::Join {
                        channels: KeywordList::new(),
                        keys: KeywordList::new(),
                    })
                } else {
                    Ok(Command::Join {
                        channels: args[0].parse()?,
                        keys: KeywordList::new(),
                    })
                }
            }
            ("JOIN", 2) => Ok(Command::Join {
                channels: args[0].parse()?,
                keys: args[1].parse()?,
            }),
            ("PART", 1) => Ok(Command::Part {
                channels: args[0].parse()?,
                message: None,
            }),
            ("PART", 2) => Ok(Command::Part {
                channels: args[0].parse()?,
                message: Some(args[1].to_string()),
            }),
            ("TOPIC", 1) => Ok(Command::Topic {
                channel: args[0].parse()?,
                topic: None,
            }),
            ("TOPIC", 2) => Ok(Command::Topic {
                channel: args[0].parse()?,
                topic: Some(args[1].to_string()),
            }),
            ("NAMES", 1) => Ok(Command::Names {
                channels: args[0].parse()?,
                target: None,
            }),
            ("NAMES", 2) => Ok(Command::Names {
                channels: args[0].parse()?,
                target: Some(args[1].parse()?),
            }),
            ("LIST", 0) => Ok(Command::List {
                channels: KeywordList::new(),
                target: None,
            }),
            ("LIST", 1) => Ok(Command::List {
                channels: args[0].parse()?,
                target: None,
            }),
            ("LIST", 2) => Ok(Command::List {
                channels: args[0].parse()?,
                target: Some(args[1].parse()?),
            }),
            ("INVITE", 2) => Ok(Command::Invite {
                nickname: args[0].parse()?,
                channel: args[1].parse()?,
            }),
            ("KICK", 2) => Ok(Command::Kick {
                channels: args[0].parse()?,
                users: args[1].parse()?,
                comment: None,
            }),
            ("KICK", 3) => Ok(Command::Kick {
                channels: args[0].parse()?,
                users: args[1].parse()?,
                comment: Some(args[2].to_string()),
            }),
            ("PRIVMSG", 2) => Ok(Command::Privmsg {
                recipients: args[0].parse()?,
                message: args[1].to_string(),
            }),
            ("NOTICE", 2) => Ok(Command::Notice {
                recipients: args[0].parse()?,
                message: args[1].to_string(),
            }),
            ("MOTD", 0) => Ok(Command::Motd { target: None }),
            ("MOTD", 1) => Ok(Command::Motd {
                target: Some(args[0].parse()?),
            }),
            ("LUSERS", 0) => Ok(Command::LUsers {
                mask: None,
                target: None,
            }),
            ("LUSERS", 1) => Ok(Command::LUsers {
                mask: Some(args[0].parse()?),
                target: None,
            }),
            ("LUSERS", 2) => Ok(Command::LUsers {
                mask: Some(args[0].parse()?),
                target: Some(args[1].parse()?),
            }),
            ("VERSION", 0) => Ok(Command::Version { target: None }),
            ("VERSION", 1) => Ok(Command::Version {
                target: Some(args[0].parse()?),
            }),
            ("STATS", 0) => Ok(Command::Stats {
                query: None,
                target: None,
            }),
            ("STATS", 1) => Ok(Command::Stats {
                query: Some(args[0].parse()?),
                target: None,
            }),
            ("STATS", 2) => Ok(Command::Stats {
                query: Some(args[0].parse()?),
                target: Some(args[1].parse()?),
            }),
            ("LINKS", 0) => Ok(Command::Links {
                mask: None,
                target: None,
            }),
            ("LINKS", 1) => Ok(Command::Links {
                mask: Some(args[0].parse()?),
                target: None,
            }),
            ("LINKS", 2) => Ok(Command::Links {
                mask: Some(args[1].parse()?),
                target: Some(args[0].parse()?),
            }),
            ("TIME", 0) => Ok(Command::Time { target: None }),
            ("TIME", 1) => Ok(Command::Time {
                target: Some(args[0].parse()?),
            }),
            ("CONNECT", 2) => Ok(Command::Connect {
                target: args[0].parse()?,
                port: args[1].parse().map_err(|_| ParseError::new("Command"))?,
                remote: None,
            }),
            ("CONNECT", 3) => Ok(Command::Connect {
                target: args[0].parse()?,
                port: args[1].parse().map_err(|_| ParseError::new("Command"))?,
                remote: Some(args[2].parse()?),
            }),
            ("TRACE", 0) => Ok(Command::Trace { target: None }),
            ("TRACE", 1) => Ok(Command::Trace {
                target: Some(args[0].parse()?),
            }),
            ("ADMIN", 0) => Ok(Command::Admin { target: None }),
            ("ADMIN", 1) => Ok(Command::Admin {
                target: Some(args[0].parse()?),
            }),
            ("INFO", 0) => Ok(Command::Info { target: None }),
            ("INFO", 1) => Ok(Command::Info {
                target: Some(args[0].parse()?),
            }),
            ("SERVLIST", 0) => Ok(Command::ServList {
                mask: None,
                service_type: None,
            }),
            ("SERVLIST", 1) => Ok(Command::ServList {
                mask: Some(args[0].to_string()),
                service_type: None,
            }),
            ("SERVLIST", 2) => Ok(Command::ServList {
                mask: Some(args[0].to_string()),
                service_type: Some(args[1].to_string()),
            }),
            ("SQUERY", 2) => Ok(Command::SQuery {
                recipient: args[0].parse()?,
                message: args[1].to_string(),
            }),
            ("WHO", 1) => Ok(Command::Who {
                mask: args[0].to_string(),
                op_only: false,
            }),
            ("WHO", 2) => Ok(Command::Who {
                mask: args[0].to_string(),
                op_only: args[1] == "o" || return Err(ParseError::new("Command")),
            }),
            ("WHOIS", 1) => Ok(Command::WhoIs {
                mask: args[0].to_string(),
                target: None,
            }),
            ("WHOIS", 2) => Ok(Command::WhoIs {
                mask: args[0].to_string(),
                target: Some(args[1].parse()?),
            }),
            ("WHOWAS", 1) => Ok(Command::WhoWas {
                targets: args[0].parse()?,
                count: None,
                target: None,
            }),
            ("WHOWAS", 2) => Ok(Command::WhoWas {
                targets: args[0].parse()?,
                count: Some(args[1].parse().map_err(|_| ParseError::new("Command"))?),
                target: None,
            }),
            ("WHOWAS", 3) => Ok(Command::WhoWas {
                targets: args[0].parse()?,
                count: Some(args[1].parse().map_err(|_| ParseError::new("Command"))?),
                target: Some(args[2].parse()?),
            }),
            ("KILL", 2) => Ok(Command::Kill {
                nickname: args[0].parse()?,
                comment: args[1].to_string(),
            }),
            ("PING", 0) => Ok(Command::Ping {
                from: None,
                to: None,
            }),
            ("PING", 1) => Ok(Command::Ping {
                from: None,
                to: Some(args[0].parse()?),
            }),
            ("PING", 2) => Ok(Command::Ping {
                from: Some(args[0].parse()?),
                to: Some(args[1].parse()?),
            }),
            ("PONG", 1) => Ok(Command::Pong {
                from: args[0].parse()?,
                to: None,
            }),
            ("PONG", 2) => Ok(Command::Pong {
                from: args[0].parse()?,
                to: Some(args[1].parse()?),
            }),
            ("ERROR", 1) => Ok(Command::Error {
                message: args[0].to_string(),
            }),
            ("AWAY", 0) => Ok(Command::Away { message: None }),
            ("AWAY", 1) => Ok(Command::Away {
                message: Some(args[0].to_string()),
            }),
            ("REHASH", 0) => Ok(Command::Rehash),
            ("DIE", 0) => Ok(Command::Die),
            ("RESTART", 0) => Ok(Command::Restart),
            ("SUMMON", 1) => Ok(Command::Summon {
                user: args[0].parse()?,
                target: None,
                channel: None,
            }),
            ("SUMMON", 2) => Ok(Command::Summon {
                user: args[0].parse()?,
                target: Some(args[1].parse()?),
                channel: None,
            }),
            ("SUMMON", 3) => Ok(Command::Summon {
                user: args[0].parse()?,
                target: Some(args[1].parse()?),
                channel: Some(args[2].parse()?),
            }),
            ("USERS", 0) => Ok(Command::Users { target: None }),
            ("USERS", 1) => Ok(Command::Users {
                target: Some(args[0].parse()?),
            }),
            ("WALLOPS", 1) => Ok(Command::WallOps {
                message: args[0].to_string(),
            }),
            ("USERHOST", 1) => Ok(Command::UserHost {
                nicknames: args[0].parse()?,
            }),
            ("ISON", 1) => Ok(Command::IsOn {
                nicknames: args[0].parse()?,
            }),
            (_, _) => Err(ParseError::new("Command")),
        }
    }
}

impl From<Command> for String {
    fn from(command: Command) -> String {
        match command {
            Command::Pass { password } => format!("PASS {}", password),
            Command::Nick { nickname } => format!("NICK {}", String::from(nickname)),
            Command::User {
                username,
                mode,
                realname,
            } => format!("USER {} {} * :{}", String::from(username), mode, realname),
            Command::Ping {
                to: Some(to),
                from: Some(from),
            } => format!("PING {} {}", String::from(from), String::from(to)),
            Command::Ping {
                to: Some(to),
                from: None,
            } => format!("PING {}", String::from(to)),
            Command::Ping {
                to: None,
                from: None,
            } => "PING".to_string(),
            Command::Pong { to: Some(to), from } => {
                format!("PONG {} {}", String::from(from), String::from(to))
            }
            Command::Pong { to: None, from } => format!("PONG {}", String::from(from)),
            _ => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_from_pass() {
        assert_eq!(
            "PASS mysecretpass".to_string(),
            String::from(Command::Pass {
                password: "mysecretpass".to_string(),
            }),
        );
    }

    #[test]
    fn string_from_nick() {
        assert_eq!(
            "NICK potato".to_string(),
            String::from(Command::Nick {
                nickname: "potato".parse().unwrap(),
            }),
        );
    }

    #[test]
    fn string_from_user() {
        assert_eq!(
            "USER pjohnson 0 * :Potato Johnson".to_string(),
            String::from(Command::User {
                username: "pjohnson".parse().unwrap(),
                mode: 0,
                realname: "Potato Johnson".to_string(),
            }),
        );
    }

    #[test]
    fn string_from_ping() {
        assert_eq!(
            "PING".to_string(),
            String::from(Command::Ping {
                to: None,
                from: None
            }),
        );
        assert_eq!(
            "PING myserver".to_string(),
            String::from(Command::Ping {
                to: Some("myserver".parse().unwrap()),
                from: None
            }),
        );
        assert_eq!(
            "PING me myserver".to_string(),
            String::from(Command::Ping {
                to: Some("myserver".parse().unwrap()),
                from: Some("me".parse().unwrap()),
            }),
        );
    }

    #[test]
    fn string_from_pong() {
        assert_eq!(
            "PONG me".to_string(),
            String::from(Command::Pong {
                from: "me".parse().unwrap(),
                to: None,
            }),
        );
        assert_eq!(
            "PONG me myserver".to_string(),
            String::from(Command::Pong {
                from: "me".parse().unwrap(),
                to: Some("myserver".parse().unwrap()),
            }),
        );
    }

    #[test]
    fn pass_from_string() {
        assert_eq!(
            Ok(Command::Pass {
                password: "mysecretpass".to_string()
            }),
            "PASS mysecretpass".parse::<Command>()
        );
    }

    #[test]
    fn nick_from_string() {
        assert_eq!(
            Ok(Command::Nick {
                nickname: "somebody".parse().unwrap(),
            }),
            "NICK somebody".parse::<Command>()
        );
    }

    #[test]
    fn user_from_string() {
        assert!("USER pjohnson 0 *".parse::<Command>().is_err());
        assert!("USER pjohnson 0 * Potato Johnson"
            .parse::<Command>()
            .is_err());

        assert_eq!(
            Ok(Command::User {
                username: "pjohnson".parse().unwrap(),
                mode: 0,
                realname: "Potato Johnson".to_string()
            }),
            "USER pjohnson 0 * :Potato Johnson".parse::<Command>()
        );
    }

    #[test]
    fn ping_from_string() {
        assert_eq!(
            Ok(Command::Ping {
                to: None,
                from: None
            }),
            "PING".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Ping {
                to: Some("myserver".parse().unwrap()),
                from: None
            }),
            "PING myserver".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Ping {
                to: Some("myserver".parse().unwrap()),
                from: Some("me".parse().unwrap())
            }),
            "PING me myserver".parse::<Command>()
        );
        assert!("PING a b c".parse::<Command>().is_err());
    }

    #[test]
    fn pong_from_string() {
        assert_eq!(
            Ok(Command::Pong {
                to: None,
                from: "me".parse().unwrap()
            }),
            "PONG me".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Pong {
                to: Some("myserver".parse().unwrap()),
                from: "me".parse().unwrap()
            }),
            "PONG me myserver".parse::<Command>()
        );
        assert!("PONG".parse::<Command>().is_err());
        assert!("PONG a b c".parse::<Command>().is_err());
    }
}
