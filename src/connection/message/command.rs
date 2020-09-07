use super::super::entity::{
    Channel, ChannelKey, Nickname, Recipient, Sender, Servername, Username,
};
use super::super::syntax::{KeywordList, ServerMask, StatsQuery};
use super::{MessageParams, ParseError};
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
        username: Username,
        mode: u8,
        realname: String,
    },
    Oper {
        user: Username,
        password: String,
    },
    UserMode {
        nickname: Nickname,
        modes: String,
    },
    Service {
        nickname: Nickname,
        distribution: ServerMask,
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
        target: Option<ServerMask>,
    },
    List {
        channels: KeywordList<Channel>,
        target: Option<ServerMask>,
    },
    Invite {
        nickname: Nickname,
        channel: Channel,
    },
    Kick {
        channels: KeywordList<Channel>,
        users: KeywordList<Username>,
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
        target: Option<ServerMask>,
    },
    LUsers {
        mask: Option<ServerMask>,
        target: Option<Servername>,
    },
    Version {
        target: Option<ServerMask>,
    },
    Stats {
        query: Option<StatsQuery>,
        target: Option<ServerMask>,
    },
    Links {
        mask: Option<ServerMask>,
        target: Option<ServerMask>,
    },
    Time {
        target: Option<ServerMask>,
    },
    Connect {
        target: Servername,
        port: u16,
        remote: Option<Servername>,
    },
    Trace {
        target: Option<String>,
    }, // TODO: add nickname
    Admin {
        target: Option<String>,
    }, // TODO: add nickname
    Info {
        target: Option<String>,
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
        mask: Option<String>,
        op_only: bool,
    },
    WhoIs {
        mask: String,
        target: Option<ServerMask>,
    },
    WhoWas {
        nicknames: KeywordList<Nickname>,
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
        user: Username,
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
                info: args[5].to_owned(),
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
            ("NAMES", 0) => Ok(Command::Names {
                channels: KeywordList::new(),
                target: None,
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
                target: Some(args[0].to_string()),
            }),
            ("ADMIN", 0) => Ok(Command::Admin { target: None }),
            ("ADMIN", 1) => Ok(Command::Admin {
                target: Some(args[0].to_string()),
            }),
            ("INFO", 0) => Ok(Command::Info { target: None }),
            ("INFO", 1) => Ok(Command::Info {
                target: Some(args[0].to_string()),
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
            ("WHO", 0) => Ok(Command::Who {
                mask: None,
                op_only: false,
            }),
            ("WHO", 1) => Ok(Command::Who {
                mask: Some(args[0].to_string()),
                op_only: false,
            }),
            ("WHO", 2) => Ok(Command::Who {
                mask: Some(args[0].to_string()),
                op_only: args[1] == "o" || return Err(ParseError::new("Command")),
            }),
            ("WHOIS", 1) => Ok(Command::WhoIs {
                mask: args[0].to_string(),
                target: None,
            }),
            ("WHOIS", 2) => Ok(Command::WhoIs {
                mask: args[1].to_string(),
                target: Some(args[0].parse()?),
            }),
            ("WHOWAS", 1) => Ok(Command::WhoWas {
                nicknames: args[0].parse()?,
                count: None,
                target: None,
            }),
            ("WHOWAS", 2) => Ok(Command::WhoWas {
                nicknames: args[0].parse()?,
                count: Some(args[1].parse().map_err(|_| ParseError::new("Command"))?),
                target: None,
            }),
            ("WHOWAS", 3) => Ok(Command::WhoWas {
                nicknames: args[0].parse()?,
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
            ("PING", 1) => {
                if raw_args.starts_with(':') {
                    Ok(Command::Ping {
                        from: Some(args[0].parse()?),
                        to: None,
                    })
                } else {
                    Ok(Command::Ping {
                        from: None,
                        to: Some(args[0].parse()?),
                    })
                }
            }
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
            ("USERHOST", 1..=15) => Ok(Command::UserHost {
                nicknames: args
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(",")
                    .parse()?,
            }),
            ("ISON", 1..=15) => Ok(Command::IsOn {
                nicknames: args
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(",")
                    .parse()?,
            }),
            _ => Err(ParseError::new("Command")),
        }
    }
}

impl From<Command> for String {
    fn from(command: Command) -> String {
        match command {
            Command::Pass { password } => {
                MessageParams::from(vec![password]).to_string_with_prefix("PASS")
            }
            Command::Nick { nickname } => {
                MessageParams::from(vec![String::from(nickname)]).to_string_with_prefix("NICK")
            }
            Command::User {
                username,
                mode,
                realname,
            } => MessageParams::from(vec![
                String::from(username),
                mode.to_string(),
                "*".to_string(),
                realname,
            ])
            .to_string_with_prefix("USER"),
            Command::Oper { user, password } => {
                MessageParams::from(vec![String::from(user), password])
                    .to_string_with_prefix("OPER")
            }
            Command::UserMode { nickname, modes } => {
                MessageParams::from(vec![String::from(nickname), modes])
                    .to_string_with_prefix("MODE")
            }
            Command::Service {
                nickname,
                distribution,
                info,
            } => MessageParams::from(vec![
                String::from(nickname),
                "".to_string(),
                String::from(distribution),
                "0".to_string(),
                "0".to_string(),
                info,
            ])
            .to_string_with_prefix("SERVICE"),
            Command::Quit { message: None } => "QUIT".to_string(),
            Command::Quit {
                message: Some(message),
            } => MessageParams::from(vec![String::from(message)]).to_string_with_prefix("QUIT"),
            Command::SQuit { server, comment } => {
                MessageParams::from(vec![String::from(server), comment])
                    .to_string_with_prefix("SQUIT")
            }

            // Channel operations
            Command::Join { channels, .. } if channels.len() == 0 => "JOIN 0".to_string(),
            Command::Join { channels, keys } if keys.len() == 0 => {
                MessageParams::from(vec![String::from(channels)]).to_string_with_prefix("JOIN")
            }
            Command::Join { channels, keys } => {
                MessageParams::from(vec![String::from(channels), String::from(keys)])
                    .to_string_with_prefix("JOIN")
            }
            Command::Part {
                channels,
                message: None,
            } => MessageParams::from(vec![String::from(channels)]).to_string_with_prefix("PART"),
            Command::Part {
                channels,
                message: Some(message),
            } => MessageParams::from(vec![String::from(channels), message])
                .to_string_with_prefix("PART"),
            Command::ChannelMode { channel, modes } => {
                format!("MODE {} {}", String::from(channel), modes)
            }
            Command::Topic {
                channel,
                topic: None,
            } => MessageParams::from(vec![String::from(channel)]).to_string_with_prefix("TOPIC"),
            Command::Topic {
                channel,
                topic: Some(topic),
            } => MessageParams::from(vec![String::from(channel), topic])
                .to_string_with_prefix("TOPIC"),
            Command::Names {
                channels,
                target: None,
            } if channels.len() == 0 => "NAMES".to_string(),
            Command::Names {
                channels,
                target: None,
            } => MessageParams::from(vec![String::from(channels)]).to_string_with_prefix("NAMES"),
            Command::Names {
                channels,
                target: Some(target),
            } => MessageParams::from(vec![String::from(channels), String::from(target)])
                .to_string_with_prefix("NAMES"),
            Command::List {
                channels,
                target: None,
            } if channels.len() == 0 => "LIST".to_string(),
            Command::List {
                channels,
                target: None,
            } => MessageParams::from(vec![String::from(channels)]).to_string_with_prefix("LIST"),
            Command::List {
                channels,
                target: Some(target),
            } => MessageParams::from(vec![String::from(channels), String::from(target)])
                .to_string_with_prefix("LIST"),
            Command::Invite { nickname, channel } => {
                MessageParams::from(vec![String::from(nickname), String::from(channel)])
                    .to_string_with_prefix("INVITE")
            }
            Command::Kick {
                channels,
                users,
                comment: None,
            } => MessageParams::from(vec![String::from(channels), String::from(users)])
                .to_string_with_prefix("KICK"),
            Command::Kick {
                channels,
                users,
                comment: Some(comment),
            } => MessageParams::from(vec![String::from(channels), String::from(users), comment])
                .to_string_with_prefix("KICK"),

            // Sending messages
            Command::Privmsg {
                recipients,
                message,
            } => MessageParams::from(vec![String::from(recipients), message])
                .to_string_with_prefix("PRIVMSG"),
            Command::Notice {
                recipients,
                message,
            } => MessageParams::from(vec![String::from(recipients), message])
                .to_string_with_prefix("NOTICE"),

            // Server queries and commands
            Command::Motd { target: None } => "MOTD".to_string(),
            Command::Motd {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("MOTD"),
            Command::LUsers {
                mask: None,
                target: None,
            } => "LUSERS".to_string(),
            Command::LUsers {
                mask: None,
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("LUSERS"),
            Command::LUsers {
                mask: Some(mask),
                target: None,
            } => MessageParams::from(vec![String::from(mask)]).to_string_with_prefix("LUSERS"),
            Command::LUsers {
                mask: Some(mask),
                target: Some(target),
            } => MessageParams::from(vec![String::from(mask), String::from(target)])
                .to_string_with_prefix("LUSERS"),
            Command::Version { target: None } => "VERSION".to_string(),
            Command::Version {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("VERSION"),
            Command::Stats { query: None, .. } => "STATS".to_string(),
            Command::Stats {
                query: Some(query),
                target: None,
            } => MessageParams::from(vec![String::from(query)]).to_string_with_prefix("STATS"),
            Command::Stats {
                query: Some(query),
                target: Some(target),
            } => MessageParams::from(vec![String::from(query), String::from(target)])
                .to_string_with_prefix("STATS"),
            Command::Links {
                mask: None,
                target: None,
            } => "LINKS".to_string(),
            Command::Links {
                mask: Some(mask),
                target: None,
            } => MessageParams::from(vec![String::from(mask)]).to_string_with_prefix("LINKS"),
            Command::Links {
                mask: None,
                target: Some(target),
            } => MessageParams::from(vec!["*".to_string(), String::from(target)])
                .to_string_with_prefix("LINKS"),
            Command::Links {
                mask: Some(mask),
                target: Some(target),
            } => MessageParams::from(vec![String::from(target), String::from(mask)])
                .to_string_with_prefix("LINKS"),
            Command::Time { target: None } => "TIME".to_string(),
            Command::Time {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("TIME"),
            Command::Connect {
                target,
                port,
                remote: None,
            } => MessageParams::from(vec![String::from(target), port.to_string()])
                .to_string_with_prefix("CONNECT"),
            Command::Connect {
                target,
                port,
                remote: Some(remote),
            } => MessageParams::from(vec![
                String::from(target),
                port.to_string(),
                String::from(remote),
            ])
            .to_string_with_prefix("CONNECT"),
            Command::Trace { target: None } => "TRACE".to_string(),
            Command::Trace {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("TRACE"),
            Command::Admin { target: None } => "ADMIN".to_string(),
            Command::Admin {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("ADMIN"),
            Command::Info { target: None } => "INFO".to_string(),
            Command::Info {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("INFO"),

            // Service query and commands
            Command::ServList {
                mask: None,
                service_type: None,
            } => "SERVLIST".to_string(),
            Command::ServList {
                mask: Some(mask),
                service_type: None,
            } => MessageParams::from(vec![String::from(mask)]).to_string_with_prefix("SERVLIST"),
            Command::ServList {
                mask: None,
                service_type: Some(service_type),
            } => MessageParams::from(vec![String::from(service_type)])
                .to_string_with_prefix("SERVLIST"),
            Command::ServList {
                mask: Some(mask),
                service_type: Some(service_type),
            } => MessageParams::from(vec![String::from(mask), service_type])
                .to_string_with_prefix("SERVLIST"),
            Command::SQuery { recipient, message } => {
                MessageParams::from(vec![String::from(recipient), message])
                    .to_string_with_prefix("SQUERY")
            }

            // User based queries
            Command::Who { mask: None, .. } => "WHO".to_string(),
            Command::Who {
                mask: Some(mask),
                op_only: false,
            } => MessageParams::from(vec![String::from(mask)]).to_string_with_prefix("WHO"),
            Command::Who {
                mask: Some(mask),
                op_only: true,
            } => MessageParams::from(vec![String::from(mask), "o".to_string()])
                .to_string_with_prefix("WHO"),
            Command::WhoIs { mask, target: None } => {
                MessageParams::from(vec![String::from(mask)]).to_string_with_prefix("WHOIS")
            }
            Command::WhoIs {
                mask,
                target: Some(target),
            } => MessageParams::from(vec![String::from(target), String::from(mask)])
                .to_string_with_prefix("WHOIS"),
            Command::WhoWas {
                nicknames,
                count: None,
                target: None,
            } => MessageParams::from(vec![String::from(nicknames)]).to_string_with_prefix("WHOWAS"),
            Command::WhoWas {
                nicknames,
                count: None,
                target: Some(target),
            } => MessageParams::from(vec![String::from(nicknames), String::from(target)])
                .to_string_with_prefix("WHOWAS"),
            Command::WhoWas {
                nicknames,
                count: Some(count),
                target: None,
            } => MessageParams::from(vec![String::from(nicknames), count.to_string()])
                .to_string_with_prefix("WHOWAS"),
            Command::WhoWas {
                nicknames,
                count: Some(count),
                target: Some(target),
            } => MessageParams::from(vec![
                String::from(nicknames),
                count.to_string(),
                String::from(target),
            ])
            .to_string_with_prefix("WHOWAS"),

            // Miscellaneous messages
            Command::Kill { nickname, comment } => {
                MessageParams::from(vec![String::from(nickname), comment])
                    .to_string_with_prefix("KILL")
            }
            Command::Ping {
                from: None,
                to: None,
            } => "PING".to_string(),
            Command::Ping {
                from: None,
                to: Some(to),
            } => MessageParams::from(vec![String::from(to)]).to_string_with_prefix("PING"),
            Command::Ping {
                from: Some(from),
                to: None,
            } => format!("PING :{}", String::from(from)),
            Command::Ping {
                from: Some(from),
                to: Some(to),
            } => MessageParams::from(vec![String::from(from), String::from(to)])
                .to_string_with_prefix("PING"),
            Command::Pong { from, to: None } => {
                MessageParams::from(vec![String::from(from)]).to_string_with_prefix("PONG")
            }
            Command::Pong { from, to: Some(to) } => {
                MessageParams::from(vec![String::from(from), String::from(to)])
                    .to_string_with_prefix("PONG")
            }
            Command::Error { message } => {
                MessageParams::from(vec![String::from(message)]).to_string_with_prefix("ERROR")
            }

            // Optional features
            Command::Away { message: None } => "AWAY".to_string(),
            Command::Away {
                message: Some(message),
            } => MessageParams::from(vec![String::from(message)]).to_string_with_prefix("AWAY"),
            Command::Rehash => "REHASH".to_string(),
            Command::Die => "DIE".to_string(),
            Command::Restart => "RESTART".to_string(),
            Command::Summon {
                user,
                target: None,
                channel: None,
            } => MessageParams::from(vec![String::from(user)]).to_string_with_prefix("SUMMON"),
            Command::Summon {
                user,
                target: None,
                channel: Some(channel),
            } => MessageParams::from(vec![String::from(user), String::from(channel)])
                .to_string_with_prefix("SUMMON"),
            Command::Summon {
                user,
                target: Some(target),
                channel: None,
            } => MessageParams::from(vec![String::from(user), String::from(target)])
                .to_string_with_prefix("SUMMON"),
            Command::Summon {
                user,
                target: Some(target),
                channel: Some(channel),
            } => MessageParams::from(vec![
                String::from(user),
                String::from(target),
                String::from(channel),
            ])
            .to_string_with_prefix("SUMMON"),
            Command::Users { target: None } => "USERS".to_string(),
            Command::Users {
                target: Some(target),
            } => MessageParams::from(vec![String::from(target)]).to_string_with_prefix("USERS"),
            Command::WallOps { message } => {
                MessageParams::from(vec![String::from(message)]).to_string_with_prefix("WALLOPS")
            }
            Command::UserHost { nicknames } => MessageParams::from(vec![String::from(nicknames)])
                .to_string_with_prefix("USERHOST")
                .replace(',', " "),
            Command::IsOn { nicknames } => MessageParams::from(vec![String::from(nicknames)])
                .to_string_with_prefix("ISON")
                .replace(',', " "),
        }
    }
}

/// The tests in this section, and their annotations, are copied verbatim from the examples in RFC 2812.
#[cfg(test)]
mod tests {
    use super::super::{Message, MessageBody};
    use super::*;

    fn assert_roundtrip(raw: &str, sender: Option<Sender>, command: Command) {
        let parsed_message = raw.parse::<Message>();
        assert_eq!(
            Ok(Message {
                sender,
                body: MessageBody::Command(command)
            }),
            parsed_message
        );
        assert_eq!(raw.to_string(), String::from(parsed_message.unwrap()),);
    }

    #[test]
    fn connection_registration_password() {
        assert_roundtrip(
            "PASS secretpasswordhere",
            None,
            Command::Pass {
                password: "secretpasswordhere".to_string(),
            },
        );
    }

    #[test]
    fn connection_registration_nick() {
        // Introducing new nick "Wiz" if session is still unregistered, or user changing his nickname to "Wiz"
        assert_roundtrip(
            "NICK Wiz",
            None,
            Command::Nick {
                nickname: "Wiz".parse().unwrap(),
            },
        );
        // Server telling that WiZ changed his nickname to Kilroy.
        assert_roundtrip(
            ":WiZ!jto@tolsun.oulu.fi NICK Kilroy",
            Some("WiZ!jto@tolsun.oulu.fi".parse().unwrap()),
            Command::Nick {
                nickname: "Kilroy".parse().unwrap(),
            },
        );
    }

    #[test]
    fn connection_registration_user() {
        // User registering themselves with a username of "guest" and real name "Ronnie Reagan".
        assert_roundtrip(
            "USER guest 0 * :Ronnie Reagan",
            None,
            Command::User {
                username: "guest".parse().unwrap(),
                mode: 0,
                realname: "Ronnie Reagan".to_string(),
            },
        );
        // User registering themselves with a username of "guest" and real name "Ronnie Reagan", and asking to be set invisible.
        assert_roundtrip(
            "USER guest 8 * :Ronnie Reagan",
            None,
            Command::User {
                username: "guest".parse().unwrap(),
                mode: 8,
                realname: "Ronnie Reagan".to_string(),
            },
        );
    }

    #[test]
    fn connection_registration_oper() {
        // Attempt to register as an operator using a username of "foo" and "bar" as the password.
        assert_roundtrip(
            "OPER foo bar",
            None,
            Command::Oper {
                user: "foo".parse().unwrap(),
                password: "bar".to_string(),
            },
        );
    }

    #[test]
    fn connection_registration_user_mode() {
        // Command by WiZ to turn off reception of WALLOPS messages.
        assert_roundtrip(
            "MODE WiZ -w",
            None,
            Command::UserMode {
                nickname: "WiZ".parse().unwrap(),
                modes: "-w".to_string(),
            },
        );
        // Command from Angel to make herself invisible.
        assert_roundtrip(
            "MODE Angel +i",
            None,
            Command::UserMode {
                nickname: "Angel".parse().unwrap(),
                modes: "+i".to_string(),
            },
        );
        // WiZ 'deopping' (removing operator status).
        assert_roundtrip(
            "MODE WiZ -o",
            None,
            Command::UserMode {
                nickname: "WiZ".parse().unwrap(),
                modes: "-o".to_string(),
            },
        );
    }

    #[test]
    fn connection_registration_service() {
        // Service registering itself with a name of "dict".  This service will only be available on servers which name matches "*.fr".
        assert_roundtrip(
            "SERVICE dict * *.fr 0 0 :French Dictionary",
            None,
            Command::Service {
                nickname: "dict".parse().unwrap(),
                distribution: "*.fr".parse().unwrap(),
                info: "French Dictionary".to_string(),
            },
        );
    }

    #[test]
    fn connection_registration_quit() {
        // Preferred message format.
        assert_roundtrip(
            "QUIT :Gone to have lunch",
            None,
            Command::Quit {
                message: Some("Gone to have lunch".to_string()),
            },
        );
        // User syrk has quit IRC to have lunch.
        assert_roundtrip(
            ":syrk!kalt@millennium.stealth.net QUIT :Gone to have lunch",
            Some("syrk!kalt@millennium.stealth.net".parse().unwrap()),
            Command::Quit {
                message: Some("Gone to have lunch".to_string()),
            },
        );
    }

    #[test]
    fn connection_registration_squit() {
        // Command to uplink of the server tolson.oulu.fi to terminate its connection with comment "Bad Link".
        assert_roundtrip(
            "SQUIT tolsun.oulu.fi :Bad Link ?",
            None,
            Command::SQuit {
                server: "tolsun.oulu.fi".parse().unwrap(),
                comment: "Bad Link ?".to_string(),
            },
        );
        // Command from Trillian from to disconnect "cm22.eng.umd.edu" from the net with comment "Server out of control".
        assert_roundtrip(
            ":Trillian SQUIT cm22.eng.umd.edu :Server out of control",
            Some("Trillian".parse().unwrap()),
            Command::SQuit {
                server: "cm22.eng.umd.edu".parse().unwrap(),
                comment: "Server out of control".to_string(),
            },
        );
    }

    #[test]
    fn channel_operations_join() {
        // Command to join channel #foobar.
        assert_roundtrip(
            "JOIN #foobar",
            None,
            Command::Join {
                channels: "#foobar".parse().unwrap(),
                keys: KeywordList::new(),
            },
        );
        // Command to join channel &foo using key "fubar".
        assert_roundtrip(
            "JOIN &foo fubar",
            None,
            Command::Join {
                channels: "&foo".parse().unwrap(),
                keys: "fubar".parse().unwrap(),
            },
        );
        // Command to join channel #foo using key "fubar" and &bar using no key.
        assert_roundtrip(
            "JOIN #foo,&bar fubar",
            None,
            Command::Join {
                channels: "#foo,&bar".parse().unwrap(),
                keys: "fubar".parse().unwrap(),
            },
        );
        // Command to join channel #foo using key "fubar", and channel #bar using key "foobar".
        assert_roundtrip(
            "JOIN #foo,#bar fubar,foobar",
            None,
            Command::Join {
                channels: "#foo,#bar".parse().unwrap(),
                keys: "fubar,foobar".parse().unwrap(),
            },
        );
        // Command to join channels #foo and #bar.
        assert_roundtrip(
            "JOIN #foo,#bar",
            None,
            Command::Join {
                channels: "#foo,#bar".parse().unwrap(),
                keys: KeywordList::new(),
            },
        );
        // Leave all currently joined channels.
        assert_roundtrip(
            "JOIN 0",
            None,
            Command::Join {
                channels: KeywordList::new(),
                keys: KeywordList::new(),
            },
        );
        // JOIN message from WiZ on channel #Twilight_zone
        assert_roundtrip(
            ":WiZ!jto@tolsun.oulu.fi JOIN #Twilight_zone",
            Some("WiZ!jto@tolsun.oulu.fi".parse().unwrap()),
            Command::Join {
                channels: "#Twilight_zone".parse().unwrap(),
                keys: KeywordList::new(),
            },
        );
    }

    #[test]
    fn channel_operations_part() {
        // Command to leave channel "#twilight_zone"
        assert_roundtrip(
            "PART #twilight_zone",
            None,
            Command::Part {
                channels: "#twilight_zone".parse().unwrap(),
                message: None,
            },
        );
        // Command to leave both channels "&group5" and "#oz-ops".
        assert_roundtrip(
            "PART #oz-ops,&group5",
            None,
            Command::Part {
                channels: "#oz-ops,&group5".parse().unwrap(),
                message: None,
            },
        );
        // User WiZ leaving channel "#playzone" with the message "I lost".
        assert_roundtrip(
            ":WiZ!jto@tolsun.oulu.fi PART #playzone :I lost",
            Some("WiZ!jto@tolsun.oulu.fi".parse().unwrap()),
            Command::Part {
                channels: "#playzone".parse().unwrap(),
                message: Some("I lost".to_string()),
            },
        );
    }

    #[test]
    fn channel_operations_channel_mode() {
        // Command to make #Finnish channel moderated and 'invite-only' with user with a hostname matching *.fi automatically invited.
        assert_roundtrip(
            "MODE #Finnish +imI *!*@*.fi",
            None,
            Command::ChannelMode {
                channel: "#Finnish".parse().unwrap(),
                modes: "+imI *!*@*.fi".to_string(),
            },
        );
        // Command to give 'chanop' privileges to Kilroy on channel #Finnish.
        assert_roundtrip(
            "MODE #Finnish +o Kilroy",
            None,
            Command::ChannelMode {
                channel: "#Finnish".parse().unwrap(),
                modes: "+o Kilroy".to_string(),
            },
        );
        // Command to allow WiZ to speak on #Finnish.
        assert_roundtrip(
            "MODE #Finnish +v Wiz",
            None,
            Command::ChannelMode {
                channel: "#Finnish".parse().unwrap(),
                modes: "+v Wiz".to_string(),
            },
        );
        // Command to remove 'secret' flag from channel #Fins.
        assert_roundtrip(
            "MODE #Fins -s",
            None,
            Command::ChannelMode {
                channel: "#Fins".parse().unwrap(),
                modes: "-s".to_string(),
            },
        );
        // Command to set the channel key to "oulu".
        assert_roundtrip(
            "MODE #42 +k oulu",
            None,
            Command::ChannelMode {
                channel: "#42".parse().unwrap(),
                modes: "+k oulu".to_string(),
            },
        );
        // Command to remove the "oulu" channel key on channel "#42".
        assert_roundtrip(
            "MODE #42 -k oulu",
            None,
            Command::ChannelMode {
                channel: "#42".parse().unwrap(),
                modes: "-k oulu".to_string(),
            },
        );
        // Command to set the limit for the number of users on channel "#eu-opers" to 10.
        assert_roundtrip(
            "MODE #eu-opers +l 10",
            None,
            Command::ChannelMode {
                channel: "#eu-opers".parse().unwrap(),
                modes: "+l 10".to_string(),
            },
        );
        // User "WiZ" removing the limit for the number of users on channel "#eu- opers".
        assert_roundtrip(
            "MODE #eu-opers -l",
            None,
            Command::ChannelMode {
                channel: "#eu-opers".parse().unwrap(),
                modes: "-l".to_string(),
            },
        );
        // Command to list ban masks set for the channel "&oulu".
        assert_roundtrip(
            "MODE &oulu +b",
            None,
            Command::ChannelMode {
                channel: "&oulu".parse().unwrap(),
                modes: "+b".to_string(),
            },
        );
        // Command to prevent all users from joining.
        assert_roundtrip(
            "MODE &oulu +b *!*@*",
            None,
            Command::ChannelMode {
                channel: "&oulu".parse().unwrap(),
                modes: "+b *!*@*".to_string(),
            },
        );
        // Command to prevent any user from a hostname matching *.edu from joining, except if matching *.bu.edu
        assert_roundtrip(
            "MODE &oulu +b *!*@*.edu +e *!*@*.bu.edu",
            None,
            Command::ChannelMode {
                channel: "&oulu".parse().unwrap(),
                modes: "+b *!*@*.edu +e *!*@*.bu.edu".to_string(),
            },
        );
        // Comment to prevent any user from a hostname matching *.edu from joining, except if matching *.bu.edu
        assert_roundtrip(
            "MODE #bu +be *!*@*.edu *!*@*.bu.edu",
            None,
            Command::ChannelMode {
                channel: "#bu".parse().unwrap(),
                modes: "+be *!*@*.edu *!*@*.bu.edu".to_string(),
            },
        );
        // Command to list exception masks set for the channel "#meditation".
        assert_roundtrip(
            "MODE #meditation e",
            None,
            Command::ChannelMode {
                channel: "#meditation".parse().unwrap(),
                modes: "e".to_string(),
            },
        );
        // Command to list invitations masks set for the channel "#meditation".
        assert_roundtrip(
            "MODE #meditation I",
            None,
            Command::ChannelMode {
                channel: "#meditation".parse().unwrap(),
                modes: "I".to_string(),
            },
        );
        // Command to ask who the channel creator for "!12345ircd" is
        assert_roundtrip(
            "MODE !12345ircd O",
            None,
            Command::ChannelMode {
                channel: "!12345ircd".parse().unwrap(),
                modes: "O".to_string(),
            },
        );
    }

    #[test]
    fn channel_operations_topic() {
        // User Wiz setting the topic.
        assert_roundtrip(
            ":WiZ!jto@tolsun.oulu.fi TOPIC #test :New topic",
            Some("WiZ!jto@tolsun.oulu.fi".parse().unwrap()),
            Command::Topic {
                channel: "#test".parse().unwrap(),
                topic: Some("New topic".to_string()),
            },
        );
        // Command to set the topic on #test to "another topic".
        assert_roundtrip(
            "TOPIC #test :another topic",
            None,
            Command::Topic {
                channel: "#test".parse().unwrap(),
                topic: Some("another topic".to_string()),
            },
        );
        // Command to clear the topic on #test.
        assert_roundtrip(
            "TOPIC #test :",
            None,
            Command::Topic {
                channel: "#test".parse().unwrap(),
                topic: Some("".to_string()),
            },
        );
        // Command to check the topic for #test.
        assert_roundtrip(
            "TOPIC #test",
            None,
            Command::Topic {
                channel: "#test".parse().unwrap(),
                topic: None,
            },
        );
    }

    #[test]
    fn channel_operations_names() {
        // Command to list visible users on #twilight_zone and #42
        assert_roundtrip(
            "NAMES #twilight_zone,#42",
            None,
            Command::Names {
                channels: "#twilight_zone,#42".parse().unwrap(),
                target: None,
            },
        );
        // Command to list all visible channels and users
        assert_roundtrip(
            "NAMES",
            None,
            Command::Names {
                channels: KeywordList::new(),
                target: None,
            },
        );
    }

    #[test]
    fn channel_operations_list() {
        // Command to list all channels
        assert_roundtrip(
            "LIST",
            None,
            Command::List {
                channels: KeywordList::new(),
                target: None,
            },
        );
        // Command to list channels #twilight_zone and #42
        assert_roundtrip(
            "LIST #twilight_zone,#42",
            None,
            Command::List {
                channels: "#twilight_zone,#42".parse().unwrap(),
                target: None,
            },
        );
    }

    #[test]
    fn channel_operations_invite() {
        // Message to WiZ when he has been invited by user Angel to channel #Dust
        assert_roundtrip(
            ":Angel!wings@irc.org INVITE Wiz #Dust",
            Some("Angel!wings@irc.org".parse().unwrap()),
            Command::Invite {
                nickname: "Wiz".parse().unwrap(),
                channel: "#Dust".parse().unwrap(),
            },
        );
        // Command to invite WiZ to #Twilight_zone
        assert_roundtrip(
            "INVITE Wiz #Twilight_Zone",
            None,
            Command::Invite {
                nickname: "Wiz".parse().unwrap(),
                channel: "#Twilight_Zone".parse().unwrap(),
            },
        );
    }

    #[test]
    fn channel_operations_kick() {
        // Command to kick Matthew from &Melbourne
        assert_roundtrip(
            "KICK &Melbourne Matthew",
            None,
            Command::Kick {
                channels: "&Melbourne".parse().unwrap(),
                users: "Matthew".parse().unwrap(),
                comment: None,
            },
        );
        // Command to kick John from #Finnish using "Speaking English" as the reason (comment).
        assert_roundtrip(
            "KICK #Finnish John :Speaking English",
            None,
            Command::Kick {
                channels: "#Finnish".parse().unwrap(),
                users: "John".parse().unwrap(),
                comment: Some("Speaking English".to_string()),
            },
        );
        // KICK message on channel #Finnish from WiZ to remove John from channel
        assert_roundtrip(
            ":WiZ!jto@tolsun.oulu.fi KICK #Finnish John",
            Some("WiZ!jto@tolsun.oulu.fi".parse().unwrap()),
            Command::Kick {
                channels: "#Finnish".parse().unwrap(),
                users: "John".parse().unwrap(),
                comment: None,
            },
        );
    }

    #[test]
    fn sending_messages_privmsg() {
        // Message from Angel to Wiz.
        assert_roundtrip(
            ":Angel!wings@irc.org PRIVMSG Wiz :Are you receiving this message ?",
            Some("Angel!wings@irc.org".parse().unwrap()),
            Command::Privmsg {
                recipients: "Wiz".parse().unwrap(),
                message: "Are you receiving this message ?".parse().unwrap(),
            },
        );
        // Command to send a message to Angel.
        assert_roundtrip(
            "PRIVMSG Angel :yes I'm receiving it !",
            None,
            Command::Privmsg {
                recipients: "Angel".parse().unwrap(),
                message: "yes I'm receiving it !".parse().unwrap(),
            },
        );
        // Command to send a message to a user on server tolsun.oulu.fi with username of "jto".
        assert_roundtrip(
            "PRIVMSG jto@tolsun.oulu.fi :Hello !",
            None,
            Command::Privmsg {
                recipients: "jto@tolsun.oulu.fi".parse().unwrap(),
                message: "Hello !".parse().unwrap(),
            },
        );
        // Message to a user on server irc.stealth.net with username of "kalt", and connected from the host millennium.stealth.net.
        assert_roundtrip(
            "PRIVMSG kalt%millennium.stealth.net@irc.stealth.net :Are you a frog?",
            None,
            Command::Privmsg {
                recipients: "kalt%millennium.stealth.net@irc.stealth.net"
                    .parse()
                    .unwrap(),
                message: "Are you a frog?".parse().unwrap(),
            },
        );
        // Message to a user on the local server with username of "kalt", and connected from the host millennium.stealth.net.
        assert_roundtrip(
            "PRIVMSG kalt%millennium.stealth.net :Do you like cheese?",
            None,
            Command::Privmsg {
                recipients: "kalt%millennium.stealth.net".parse().unwrap(),
                message: "Do you like cheese?".parse().unwrap(),
            },
        );
        // Message to the user with nickname Wiz who is connected from the host tolsun.oulu.fi and has the username "jto".
        assert_roundtrip(
            "PRIVMSG Wiz!jto@tolsun.oulu.fi :Hello !",
            None,
            Command::Privmsg {
                recipients: "Wiz!jto@tolsun.oulu.fi".parse().unwrap(),
                message: "Hello !".parse().unwrap(),
            },
        );
        // Message to everyone on a server which has a name matching *.fi.
        assert_roundtrip(
            "PRIVMSG $*.fi :Server tolsun.oulu.fi rebooting.",
            None,
            Command::Privmsg {
                recipients: "$*.fi".parse().unwrap(),
                message: "Server tolsun.oulu.fi rebooting.".parse().unwrap(),
            },
        );
        // Message to all users who come from a host which has a name matching *.edu.
        assert_roundtrip(
            "PRIVMSG #*.edu :NSFNet is undergoing work, expect interruptions",
            None,
            Command::Privmsg {
                recipients: "#*.edu".parse().unwrap(),
                message: "NSFNet is undergoing work, expect interruptions"
                    .parse()
                    .unwrap(),
            },
        );
    }

    #[test]
    fn sending_messages_notice() {
        // Message from Angel to Wiz.
        assert_roundtrip(
            ":Angel!wings@irc.org NOTICE Wiz :Are you receiving this message ?",
            Some("Angel!wings@irc.org".parse().unwrap()),
            Command::Notice {
                recipients: "Wiz".parse().unwrap(),
                message: "Are you receiving this message ?".parse().unwrap(),
            },
        );
        // Message to all users who come from a host which has a name matching *.edu.
        assert_roundtrip(
            "NOTICE #*.edu :NSFNet is undergoing work, expect interruptions",
            None,
            Command::Notice {
                recipients: "#*.edu".parse().unwrap(),
                message: "NSFNet is undergoing work, expect interruptions"
                    .parse()
                    .unwrap(),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_motd() {
        assert_roundtrip("MOTD", None, Command::Motd { target: None });
        assert_roundtrip(
            "MOTD irc.example.com",
            None,
            Command::Motd {
                target: Some("irc.example.com".parse().unwrap()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_lusers() {
        assert_roundtrip(
            "LUSERS",
            None,
            Command::LUsers {
                mask: None,
                target: None,
            },
        );
        assert_roundtrip(
            ":irc.example.com LUSERS *.com",
            Some("irc.example.com".parse().unwrap()),
            Command::LUsers {
                mask: Some("*.com".parse().unwrap()),
                target: None,
            },
        );
        assert_roundtrip(
            "LUSERS *.com irc.example.com",
            None,
            Command::LUsers {
                mask: Some("*.com".parse().unwrap()),
                target: Some("irc.example.com".parse().unwrap()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_version() {
        assert_roundtrip("VERSION", None, Command::Version { target: None });
        // Command to check the version of server "tolsun.oulu.fi".
        assert_roundtrip(
            "VERSION tolsun.oulu.fi",
            None,
            Command::Version {
                target: Some("tolsun.oulu.fi".parse().unwrap()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_stats() {
        assert_roundtrip(
            "STATS",
            None,
            Command::Stats {
                query: None,
                target: None,
            },
        );
        // Command to check the command usage for the server you are connected to
        assert_roundtrip(
            "STATS m",
            None,
            Command::Stats {
                query: Some("m".parse().unwrap()),
                target: None,
            },
        );
        assert_roundtrip(
            "STATS o irc.example.com",
            None,
            Command::Stats {
                query: Some("o".parse().unwrap()),
                target: Some("irc.example.com".parse().unwrap()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_links() {
        assert_roundtrip(
            "LINKS",
            None,
            Command::Links {
                mask: None,
                target: None,
            },
        );
        // Command to list all servers which have a name that matches *.au;
        assert_roundtrip(
            "LINKS *.au",
            None,
            Command::Links {
                mask: Some("*.au".parse().unwrap()),
                target: None,
            },
        );
        // Command to list servers matching *.bu.edu as seen by the first server matching *.edu.
        assert_roundtrip(
            "LINKS *.edu *.bu.edu",
            None,
            Command::Links {
                mask: Some("*.bu.edu".parse().unwrap()),
                target: Some("*.edu".parse().unwrap()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_time() {
        assert_roundtrip("TIME", None, Command::Time { target: None });
        // check the time on the server "tolson.oulu.fi"
        assert_roundtrip(
            "TIME tolsun.oulu.fi",
            None,
            Command::Time {
                target: Some("tolsun.oulu.fi".parse().unwrap()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_trace() {
        assert_roundtrip("TRACE", None, Command::Trace { target: None });
        // TRACE to a server matching *.oulu.fi
        assert_roundtrip(
            "TRACE *.oulu.fi",
            None,
            Command::Trace {
                target: Some("*.oulu.fi".to_string()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_admin() {
        assert_roundtrip("ADMIN", None, Command::Admin { target: None });
        // request an ADMIN reply from tolsun.oulu.fi
        assert_roundtrip(
            "ADMIN tolsun.oulu.fi",
            None,
            Command::Admin {
                target: Some("tolsun.oulu.fi".to_string()),
            },
        );
        // ADMIN request for the server to which the user syrk is connected
        assert_roundtrip(
            "ADMIN syrk",
            None,
            Command::Admin {
                target: Some("syrk".to_string()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_info() {
        assert_roundtrip("INFO", None, Command::Info { target: None });
        // request an INFO reply from csd.bu.edu
        assert_roundtrip(
            "INFO csd.bu.edu",
            None,
            Command::Info {
                target: Some("csd.bu.edu".to_string()),
            },
        );
        // request info from the server that Angel is connected to.
        assert_roundtrip(
            "INFO Angel",
            None,
            Command::Info {
                target: Some("Angel".to_string()),
            },
        );
        assert_roundtrip(
            "INFO *.example.com",
            None,
            Command::Info {
                target: Some("*.example.com".to_string()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_servlist() {
        assert_roundtrip(
            "SERVLIST",
            None,
            Command::ServList {
                mask: None,
                service_type: None,
            },
        );
        assert_roundtrip(
            "SERVLIST *dict",
            None,
            Command::ServList {
                mask: Some("*dict".to_string()),
                service_type: None,
            },
        );
        assert_roundtrip(
            "SERVLIST * bot",
            None,
            Command::ServList {
                mask: Some("*".to_string()),
                service_type: Some("bot".to_string()),
            },
        );
    }

    #[test]
    fn server_queries_and_commands_squery() {
        // Message to the service with nickname irchelp.
        assert_roundtrip(
            "SQUERY irchelp :HELP privmsg",
            None,
            Command::SQuery {
                recipient: "irchelp".parse().unwrap(),
                message: "HELP privmsg".to_string(),
            },
        );
        // Message to the service with name dict@irc.fr.
        assert_roundtrip(
            "SQUERY dict@irc.fr :fr2en blaireau",
            None,
            Command::SQuery {
                recipient: "dict@irc.fr".parse().unwrap(),
                message: "fr2en blaireau".to_string(),
            },
        );
    }

    #[test]
    fn user_based_queries_who() {
        assert_roundtrip(
            "WHO",
            None,
            Command::Who {
                mask: None,
                op_only: false,
            },
        );
        // Command to list all users who match against "*.fi".
        assert_roundtrip(
            "WHO *.fi",
            None,
            Command::Who {
                mask: Some("*.fi".to_string()),
                op_only: false,
            },
        );
        // Command to list all users with a match against "jto*" if they are an operator.
        assert_roundtrip(
            "WHO jto* o",
            None,
            Command::Who {
                mask: Some("jto*".to_string()),
                op_only: true,
            },
        );
    }

    #[test]
    fn user_based_queries_whois() {
        // return available user information about nick WiZ
        assert_roundtrip(
            "WHOIS wiz",
            None,
            Command::WhoIs {
                target: None,
                mask: "wiz".to_string(),
            },
        );
        // ask server eff.org for user information  about trillian
        assert_roundtrip(
            "WHOIS eff.org trillian",
            None,
            Command::WhoIs {
                target: Some("eff.org".parse().unwrap()),
                mask: "trillian".to_string(),
            },
        );
    }

    #[test]
    fn user_based_queries_whowas() {
        // return all information in the nick history about nick "WiZ";
        assert_roundtrip(
            "WHOWAS Wiz",
            None,
            Command::WhoWas {
                nicknames: "Wiz".parse().unwrap(),
                count: None,
                target: None,
            },
        );
        // return at most, the 9 most recent entries in the nick history for "Mermaid";
        assert_roundtrip(
            "WHOWAS Mermaid 9",
            None,
            Command::WhoWas {
                nicknames: "Mermaid".parse().unwrap(),
                count: Some(9),
                target: None,
            },
        );
        // return the most recent history for "Trillian" from the first server found to match "*.edu".
        assert_roundtrip(
            "WHOWAS Trillian 1 *.edu",
            None,
            Command::WhoWas {
                nicknames: "Trillian".parse().unwrap(),
                count: Some(1),
                target: Some("*.edu".parse().unwrap()),
            },
        );
    }

    #[test]
    fn miscellaneous_messages_kill() {
        assert_roundtrip(
            "KILL Kenny :It's a trope, okay?",
            None,
            Command::Kill {
                nickname: "Kenny".parse().unwrap(),
                comment: "It's a trope, okay?".to_string(),
            },
        );
    }

    #[test]
    fn miscellaneous_messages_ping() {
        // Command to send a PING message to server
        assert_roundtrip(
            "PING tolsun.oulu.fi",
            None,
            Command::Ping {
                to: Some("tolsun.oulu.fi".parse().unwrap()),
                from: None,
            },
        );
        // Command from WiZ to send a PING message to server "tolsun.oulu.fi"
        assert_roundtrip(
            "PING WiZ tolsun.oulu.fi",
            None,
            Command::Ping {
                to: Some("tolsun.oulu.fi".parse().unwrap()),
                from: Some("WiZ".parse().unwrap()),
            },
        );
        // Ping message sent by server "irc.funet.fi"
        assert_roundtrip(
            "PING :irc.funet.fi",
            None,
            Command::Ping {
                to: None,
                from: Some("irc.funet.fi".parse().unwrap()),
            },
        );
    }

    #[test]
    fn miscellaneous_messages_pong() {
        assert_roundtrip(
            "PONG irc.example.com",
            None,
            Command::Pong {
                to: None,
                from: "irc.example.com".parse().unwrap(),
            },
        );
        // PONG message from csd.bu.edu to tolsun.oulu.fi
        assert_roundtrip(
            "PONG csd.bu.edu tolsun.oulu.fi",
            None,
            Command::Pong {
                to: Some("tolsun.oulu.fi".parse().unwrap()),
                from: "csd.bu.edu".parse().unwrap(),
            },
        );
    }

    #[test]
    fn miscellaneous_messages_error() {
        // ERROR message to the other server which caused this error.
        assert_roundtrip(
            "ERROR :Server *.fi already exists",
            None,
            Command::Error {
                message: "Server *.fi already exists".to_string(),
            },
        );
        // Same ERROR message as above but sent to user WiZ on the other server.
        assert_roundtrip(
            "NOTICE WiZ :ERROR from csd.bu.edu -- Server *.fi already exists",
            None,
            Command::Notice {
                recipients: "WiZ".parse().unwrap(),
                message: "ERROR from csd.bu.edu -- Server *.fi already exists".to_string(),
            },
        );
    }

    #[test]
    fn optional_features_away() {
        assert_roundtrip("AWAY", None, Command::Away { message: None });
        // Command to set away message to "Gone to lunch.  Back in 5".
        assert_roundtrip(
            "AWAY :Gone to lunch.  Back in 5",
            None,
            Command::Away {
                message: Some("Gone to lunch.  Back in 5".to_string()),
            },
        );
    }

    #[test]
    fn optional_features_rehash() {
        // message from user with operator status to server asking it to reread its configuration file.
        assert_roundtrip("REHASH", None, Command::Rehash);
    }

    #[test]
    fn optional_features_die() {
        // no parameters required
        assert_roundtrip("DIE", None, Command::Die);
    }

    #[test]
    fn optional_features_restart() {
        // no parameters required
        assert_roundtrip("RESTART", None, Command::Restart);
    }

    #[test]
    fn optional_features_summon() {
        // summon user jto on the server's host
        assert_roundtrip(
            "SUMMON jto",
            None,
            Command::Summon {
                user: "jto".parse().unwrap(),
                target: None,
                channel: None,
            },
        );
        // summon user jto on the host which a server named "tolsun.oulu.fi" is running.
        assert_roundtrip(
            "SUMMON jto tolsun.oulu.fi",
            None,
            Command::Summon {
                user: "jto".parse().unwrap(),
                target: Some("tolsun.oulu.fi".parse().unwrap()),
                channel: None,
            },
        );
        assert_roundtrip(
            "SUMMON spudly irc.example.com #potato",
            None,
            Command::Summon {
                user: "spudly".parse().unwrap(),
                target: Some("irc.example.com".parse().unwrap()),
                channel: Some("#potato".parse().unwrap()),
            },
        );
    }

    #[test]
    fn optional_features_users() {
        assert_roundtrip("USERS", None, Command::Users { target: None });
        // request a list of users logged in on server eff.org
        assert_roundtrip(
            "USERS eff.org",
            None,
            Command::Users {
                target: Some("eff.org".parse().unwrap()),
            },
        );
    }

    #[test]
    fn optional_features_wallops() {
        // WALLOPS message from csd.bu.edu announcing a CONNECT message it received from Joshua and acted upon.
        assert_roundtrip(
            ":csd.bu.edu WALLOPS :Connect '*.uiuc.edu 6667' from Joshua",
            Some("csd.bu.edu".parse().unwrap()),
            Command::WallOps {
                message: "Connect '*.uiuc.edu 6667' from Joshua".to_string(),
            },
        );
    }

    #[test]
    fn optional_features_userhost() {
        // USERHOST request for information on nicks "Wiz", "Michael", and "syrk"
        assert_roundtrip(
            "USERHOST Wiz Michael syrk",
            None,
            Command::UserHost {
                nicknames: "Wiz,Michael,syrk".parse().unwrap(),
            },
        );
    }

    #[test]
    fn optional_features_ison() {
        // Sample ISON request for 7 nicks.
        assert_roundtrip(
            "ISON phone trillian WiZ jarlek Avalon Angel Monstah syrk",
            None,
            Command::IsOn {
                nicknames: "phone,trillian,WiZ,jarlek,Avalon,Angel,Monstah,syrk"
                    .parse()
                    .unwrap(),
            },
        );
    }
}
