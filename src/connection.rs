use std::io;
use std::io::prelude::*;
use std::net;
use std::str::FromStr;

pub trait Connect {
    fn poll(&mut self) -> bool;

    fn send_command(&mut self, command: Command) -> std::io::Result<()>;

    fn send_command_raw(&mut self, raw_command: String) -> std::io::Result<()>;
}

pub struct Connection<'a> {
    reader: Box<dyn 'a + io::BufRead>,
    writer: Box<dyn 'a + Write>,
}

impl<'a> Connection<'a> {
    pub fn new(stream: &'a net::TcpStream) -> Connection<'a> {
        stream.set_nonblocking(true).unwrap();

        let reader = io::BufReader::new(stream);

        Connection {
            reader: Box::new(reader),
            writer: Box::new(stream),
        }
    }
}

impl<'a> Connect for Connection<'a> {
    fn poll(&mut self) -> bool {
        let mut buffer = String::new();

        match self.reader.read_line(&mut buffer) {
            Ok(len) => {
                if len == 0 {
                    panic!("Stream disconnected");
                } else {
                    print!("< {}", buffer);
                    true
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => false,
            Err(e) => panic!("IO error: {}", e),
        }
    }

    fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        let raw_command = command_to_raw(command);
        self.send_command_raw(raw_command)
    }

    fn send_command_raw(&mut self, mut raw_command: String) -> std::io::Result<()> {
        raw_command.push_str("\r\n");
        print!("> {}", raw_command);
        self.writer.write(raw_command.as_bytes())?;
        Ok(())
    }
}

fn split_server_name(raw_message: &mut String) -> Option<String> {
    if raw_message.starts_with(':') {
        let slice_index = raw_message.find(' ')?;
        let server_name = raw_message[1..slice_index].to_string();
        raw_message.replace_range(..slice_index + 1, "");
        Some(server_name)
    } else {
        None
    }
}

fn raw_to_command(raw_command: &str) -> Option<Command> {
    let command_parts: Vec<&str> = raw_command.split(' ').collect();

    match command_parts.first()?.as_ref() {
        "PASS" => {
            if command_parts.len() == 2 {
                Some(Command::Pass {
                    password: command_parts[1].to_string(),
                })
            } else {
                None
            }
        }
        "NICK" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Nick {
                    nickname: command_parts[1].to_string(),
                    hopcount: match command_parts.get(2) {
                        Some(n) => u8::from_str(n).ok(),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        "USER" => {
            if command_parts.len() >= 5 {
                Some(Command::User {
                    username: command_parts[1].to_string(),
                    hostname: command_parts[2].to_string(),
                    servername: command_parts[3].to_string(),
                    realname: command_parts[4..].join(" ").strip_prefix(":")?.to_string(),
                })
            } else {
                None
            }
        }
        "PING" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Ping {
                    server1: command_parts[1].to_string(),
                    server2: match command_parts.get(2) {
                        Some(&server2) => Some(server2.to_string()),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        "PONG" => {
            if command_parts.len() >= 2 && command_parts.len() <= 3 {
                Some(Command::Pong {
                    server1: command_parts[1].to_string(),
                    server2: match command_parts.get(2) {
                        Some(&server2) => Some(server2.to_string()),
                        None => None,
                    },
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn command_to_raw(command: Command) -> String {
    match command {
        Command::Pass { password } => format!("PASS {}", password),
        Command::Nick { nickname, hopcount } => match hopcount {
            Some(hopcount) => format!("NICK {} {}", nickname, hopcount),
            None => format!("NICK {}", nickname),
        },
        Command::User {
            username,
            hostname,
            servername,
            realname,
        } => format!(
            "USER {} {} {} :{}",
            username, hostname, servername, realname
        ),
        Command::Ping { server1, server2 } => match server2 {
            Some(server2) => format!("PING {} {}", server1, server2),
            None => format!("PING {}", server1),
        },
        Command::Pong { server1, server2 } => match server2 {
            Some(server2) => format!("PONG {} {}", server1, server2),
            None => format!("PONG {}", server1),
        },
    }
}

fn raw_to_reply(raw_reply: &str) -> Option<(ReplyType, String)> {
    let reply_parts: Vec<&str> = raw_reply.split(' ').collect();

    let (reply_prefix, reply_body_parts) = reply_parts.split_first()?;
    let reply_body = reply_body_parts.join(" ");

    Some((
        match *reply_prefix {
            "001" => ReplyType::RplWelcome,
            "002" => ReplyType::RplYourHost,
            "003" => ReplyType::RplCreated,
            "004" => ReplyType::RplMyInfo,
            "005" => ReplyType::RplBounce,
            "302" => ReplyType::RplUserHost,
            "303" => ReplyType::RplIsOn,
            "301" => ReplyType::RplAway,
            "305" => ReplyType::RplUnAway,
            "306" => ReplyType::RplNowAway,
            "311" => ReplyType::RplWhoIsUser,
            "312" => ReplyType::RplWhoIsServer,
            "313" => ReplyType::RplWhoIsOperator,
            "317" => ReplyType::RplWhoIsIdle,
            "318" => ReplyType::RplEndOfWhoIs,
            "319" => ReplyType::RplWhoIsChannels,
            "314" => ReplyType::RplWhoWasUser,
            "369" => ReplyType::RplEndOfWhoWas,
            "321" => ReplyType::RplListStart,
            "322" => ReplyType::RplList,
            "323" => ReplyType::RplListEnd,
            "325" => ReplyType::RplUniqOpIs,
            "324" => ReplyType::RplChannelModeIs,
            "331" => ReplyType::RplNoTopic,
            "332" => ReplyType::RplTopic,
            "341" => ReplyType::RplInviting,
            "342" => ReplyType::RplSummoning,
            "346" => ReplyType::RplInviteList,
            "347" => ReplyType::RplEndOfInviteList,
            "348" => ReplyType::RplExceptList,
            "349" => ReplyType::RplEndOfExceptList,
            "351" => ReplyType::RplVersion,
            "352" => ReplyType::RplWhoReply,
            "315" => ReplyType::RplEndOfWho,
            "353" => ReplyType::RplNamReply,
            "366" => ReplyType::RplEndOfNames,
            "364" => ReplyType::RplLinks,
            "365" => ReplyType::RplEndOfLinks,
            "367" => ReplyType::RplBanList,
            "368" => ReplyType::RplEndOfBanList,
            "371" => ReplyType::RplInfo,
            "374" => ReplyType::RplEndOfInfo,
            "375" => ReplyType::RplMotdStart,
            "372" => ReplyType::RplMotd,
            "376" => ReplyType::RplEndOfMotd,
            "381" => ReplyType::RplYoureOper,
            "382" => ReplyType::RplRehashing,
            "383" => ReplyType::RplYoureService,
            "391" => ReplyType::RplTime,
            "392" => ReplyType::RplUsersStart,
            "393" => ReplyType::RplUsers,
            "394" => ReplyType::RplEndOfUsers,
            "395" => ReplyType::RplNoUsers,
            "200" => ReplyType::RplTraceLink,
            "201" => ReplyType::RplTraceConnecting,
            "202" => ReplyType::RplTraceHandshake,
            "203" => ReplyType::RplTraceUnknown,
            "204" => ReplyType::RplTraceOperator,
            "205" => ReplyType::RplTraceUser,
            "206" => ReplyType::RplTraceServer,
            "207" => ReplyType::RplTraceService,
            "208" => ReplyType::RplTraceNewType,
            "209" => ReplyType::RplTraceClass,
            "210" => ReplyType::RplTraceReconnect,
            "261" => ReplyType::RplTraceLog,
            "262" => ReplyType::RplTraceEnd,
            "211" => ReplyType::RplStatsLinkInfo,
            "212" => ReplyType::RplStatsCommands,
            "219" => ReplyType::RplEndOfStats,
            "242" => ReplyType::RplStatsUptime,
            "243" => ReplyType::RplStatsOLine,
            "221" => ReplyType::RplUModeIs,
            "234" => ReplyType::RplServList,
            "235" => ReplyType::RplServListEnd,
            "251" => ReplyType::RplLUserClient,
            "252" => ReplyType::RplLUserOp,
            "253" => ReplyType::RplLUserUnknown,
            "254" => ReplyType::RplLUserChannels,
            "255" => ReplyType::RplLUserMe,
            "256" => ReplyType::RplAdminMe,
            "257" => ReplyType::RplAdminLoc1,
            "258" => ReplyType::RplAdminLoc2,
            "259" => ReplyType::RplAdminEmail,
            "263" => ReplyType::RplTryAgain,
            "401" => ReplyType::ErrNoSuchNick,
            "402" => ReplyType::ErrNoSuchServer,
            "403" => ReplyType::ErrNoSuchChannel,
            "404" => ReplyType::ErrCannotSendToChan,
            "405" => ReplyType::ErrTooManyChannels,
            "406" => ReplyType::ErrWasNoSuchNick,
            "407" => ReplyType::ErrTooManyTargets,
            "408" => ReplyType::ErrNoSuchService,
            "409" => ReplyType::ErrNoOrigin,
            "411" => ReplyType::ErrNoRecipient,
            "412" => ReplyType::ErrNoTextToSend,
            "413" => ReplyType::ErrNoTopLevel,
            "414" => ReplyType::ErrWildTopLevel,
            "415" => ReplyType::ErrBadMask,
            "421" => ReplyType::ErrUnknownCommand,
            "422" => ReplyType::ErrNoMotd,
            "423" => ReplyType::ErrNoAdminInfo,
            "424" => ReplyType::ErrFileError,
            "431" => ReplyType::ErrNoNicknameGiven,
            "432" => ReplyType::ErrErroneusNickname,
            "433" => ReplyType::ErrNicknameInUse,
            "436" => ReplyType::ErrNickCollision,
            "437" => ReplyType::ErrUnavailResource,
            "441" => ReplyType::ErrUserNotInChannel,
            "442" => ReplyType::ErrNotOnChannel,
            "443" => ReplyType::ErrUserOnChannel,
            "444" => ReplyType::ErrNoLogin,
            "445" => ReplyType::ErrSummonDisabled,
            "446" => ReplyType::ErrUsersDisabled,
            "451" => ReplyType::ErrNotRegistered,
            "461" => ReplyType::ErrNeedMoreParams,
            "462" => ReplyType::ErrAlreadyRegistred,
            "463" => ReplyType::ErrNoPermForHost,
            "464" => ReplyType::ErrPasswdMismatch,
            "465" => ReplyType::ErrYoureBannedCreep,
            "466" => ReplyType::ErrYouWillBeBanned,
            "467" => ReplyType::ErrKeySet,
            "471" => ReplyType::ErrChannelIsFull,
            "472" => ReplyType::ErrUnknownMode,
            "473" => ReplyType::ErrInviteOnlyChan,
            "474" => ReplyType::ErrBannedFromChan,
            "475" => ReplyType::ErrBadChannelKey,
            "476" => ReplyType::ErrBadChanMask,
            "477" => ReplyType::ErrNoChanModes,
            "478" => ReplyType::ErrBanListFull,
            "481" => ReplyType::ErrNoPrivileges,
            "482" => ReplyType::ErrChanOPrivsNeeded,
            "483" => ReplyType::ErrCantKillServer,
            "484" => ReplyType::ErrRestricted,
            "485" => ReplyType::ErrUniqOpPrivsNeeded,
            "491" => ReplyType::ErrNoOperHost,
            "501" => ReplyType::ErrUModeUnknownFlag,
            "502" => ReplyType::ErrUsersDontMatch,
            _ => return None,
        },
        reply_body,
    ))
}

#[derive(Debug)]
pub enum Command {
    // Connection registration
    Pass {
        password: String,
    },
    Nick {
        nickname: String,
        hopcount: Option<u8>,
    },
    User {
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    },
    //Oper { user: String, password: String },
    //Quit { message: Option<String> },

    // Channel operations
    //Join { channels: Vec<String>, keys: Vec<String> },
    //Part { channels: Vec<String> },

    // Sending messages
    //Privmsg { receivers: Vec<Messageable>, message: String },
    //Notice { receivers: Vec<Messageable>, message: String },

    // Miscellaneous messages
    Ping {
        server1: String,
        server2: Option<String>,
    },
    Pong {
        server1: String,
        server2: Option<String>,
    },
}

impl Command {
    pub fn to_command_type(&self) -> CommandType {
        match self {
            Command::Pass { .. } => CommandType::Pass,
            Command::Nick { .. } => CommandType::Nick,
            Command::User { .. } => CommandType::User,
            Command::Ping { .. } => CommandType::Ping,
            Command::Pong { .. } => CommandType::Pong,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CommandType {
    // Connection registration
    Pass,
    Nick,
    User,
    //Oper,
    //Quit,

    // Channel operations
    //Join,
    //Part,

    // Sending messages
    //Privmsg,
    //Notice,

    // Miscellaneous messages
    Ping,
    Pong,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ReplyType {
    RplWelcome,           // 001 - "Welcome to the Internet Relay Network
    RplYourHost,          // 002 - "Your host is <servername>, running version <ver>"
    RplCreated,           // 003 - "This server was created <date>"
    RplMyInfo,            // 004 - "<servername> <version> <available user modes>
    RplBounce,            // 005 - "Try server <server name>, port <port number>"
    RplUserHost,          // 302 - ":*1<reply> *( " " <reply> )"
    RplIsOn,              // 303 - ":*1<nick> *( " " <nick> )"
    RplAway,              // 301 - "<nick> :<away message>"
    RplUnAway,            // 305 - ":You are no longer marked as being away"
    RplNowAway,           // 306 - ":You have been marked as being away"
    RplWhoIsUser,         // 311 - "<nick> <user> <host> * :<real name>"
    RplWhoIsServer,       // 312 - "<nick> <server> :<server info>"
    RplWhoIsOperator,     // 313 - "<nick> :is an IRC operator"
    RplWhoIsIdle,         // 317 - "<nick> <integer> :seconds idle"
    RplEndOfWhoIs,        // 318 - "<nick> :End of WHOIS list"
    RplWhoIsChannels,     // 319 - "<nick> :*( ( "@" / "+" ) <channel> " " )"
    RplWhoWasUser,        // 314 - "<nick> <user> <host> * :<real name>"
    RplEndOfWhoWas,       // 369 - "<nick> :End of WHOWAS"
    RplListStart,         // 321 - Obsolete. Not used.
    RplList,              // 322 - "<channel> <# visible> :<topic>"
    RplListEnd,           // 323 - ":End of LIST"
    RplUniqOpIs,          // 325 - "<channel> <nickname>"
    RplChannelModeIs,     // 324 - "<channel> <mode> <mode params>"
    RplNoTopic,           // 331 - "<channel> :No topic is set"
    RplTopic,             // 332 - "<channel> :<topic>"
    RplInviting,          // 341 - "<channel> <nick>"
    RplSummoning,         // 342 - "<user> :Summoning user to IRC"
    RplInviteList,        // 346 - "<channel> <invitemask>"
    RplEndOfInviteList,   // 347 - "<channel> :End of channel invite list"
    RplExceptList,        // 348 - "<channel> <exceptionmask>"
    RplEndOfExceptList,   // 349 - "<channel> :End of channel exception list"
    RplVersion,           // 351 - "<version>.<debuglevel> <server> :<comments>"
    RplWhoReply,          // 352 - "<channel> <user> <host> <server> <nick>
    RplEndOfWho,          // 315 - "<name> :End of WHO list"
    RplNamReply,          // 353 - "( "=" / "*" / "@" ) <channel>
    RplEndOfNames,        // 366 - "<channel> :End of NAMES list"
    RplLinks,             // 364 - "<mask> <server> :<hopcount> <server info>"
    RplEndOfLinks,        // 365 - "<mask> :End of LINKS list"
    RplBanList,           // 367 - "<channel> <banmask>"
    RplEndOfBanList,      // 368 - "<channel> :End of channel ban list"
    RplInfo,              // 371 - ":<string>"
    RplEndOfInfo,         // 374 - ":End of INFO list"
    RplMotdStart,         // 375 - ":- <server> Message of the day - "
    RplMotd,              // 372 - ":- <text>"
    RplEndOfMotd,         // 376 - ":End of MOTD command"
    RplYoureOper,         // 381 - ":You are now an IRC operator"
    RplRehashing,         // 382 - "<config file> :Rehashing"
    RplYoureService,      // 383 - "You are service <servicename>"
    RplTime,              // 391 - "<server> :<string showing server's local time>"
    RplUsersStart,        // 392 - ":UserID   Terminal  Host"
    RplUsers,             // 393 - ":<username> <ttyline> <hostname>"
    RplEndOfUsers,        // 394 - ":End of users"
    RplNoUsers,           // 395 - ":Nobody logged in"
    RplTraceLink,         // 200 - "Link <version & debug level> <destination>
    RplTraceConnecting,   // 201 - "Try. <class> <server>"
    RplTraceHandshake,    // 202 - "H.S. <class> <server>"
    RplTraceUnknown,      // 203 - "???? <class> [<client IP address in dot form>]"
    RplTraceOperator,     // 204 - "Oper <class> <nick>"
    RplTraceUser,         // 205 - "User <class> <nick>"
    RplTraceServer,       // 206 - "Serv <class> <int>S <int>C <server>
    RplTraceService,      // 207 - "Service <class> <name> <type> <active type>"
    RplTraceNewType,      // 208 - "<newtype> 0 <client name>"
    RplTraceClass,        // 209 - "Class <class> <count>"
    RplTraceReconnect,    // 210 - Unused.
    RplTraceLog,          // 261 - "File <logfile> <debug level>"
    RplTraceEnd,          // 262 - "<server name> <version & debug level> :End of TRACE"
    RplStatsLinkInfo,     // 211 - "<linkname> <sendq> <sent messages>
    RplStatsCommands,     // 212 - "<command> <count> <byte count> <remote count>"
    RplEndOfStats,        // 219 - "<stats letter> :End of STATS report"
    RplStatsUptime,       // 242 - ":Server Up %d days %d:%02d:%02d"
    RplStatsOLine,        // 243 - "O <hostmask> * <name>"
    RplUModeIs,           // 221 - "<user mode string>"
    RplServList,          // 234 - "<name> <server> <mask> <type> <hopcount> <info>"
    RplServListEnd,       // 235 - "<mask> <type> :End of service listing"
    RplLUserClient,       // 251 - ":There are <integer> users and <integer>
    RplLUserOp,           // 252 - "<integer> :operator(s) online"
    RplLUserUnknown,      // 253 - "<integer> :unknown connection(s)"
    RplLUserChannels,     // 254 - "<integer> :channels formed"
    RplLUserMe,           // 255 - ":I have <integer> clients and <integer>
    RplAdminMe,           // 256 - "<server> :Administrative info"
    RplAdminLoc1,         // 257 - ":<admin info>"
    RplAdminLoc2,         // 258 - ":<admin info>"
    RplAdminEmail,        // 259 - ":<admin info>"
    RplTryAgain,          // 263 - "<command> :Please wait a while and try again."
    ErrNoSuchNick,        // 401 - "<nickname> :No such nick/channel"
    ErrNoSuchServer,      // 402 - "<server name> :No such server"
    ErrNoSuchChannel,     // 403 - "<channel name> :No such channel"
    ErrCannotSendToChan,  // 404 - "<channel name> :Cannot send to channel"
    ErrTooManyChannels,   // 405 - "<channel name> :You have joined too many channels"
    ErrWasNoSuchNick,     // 406 - "<nickname> :There was no such nickname"
    ErrTooManyTargets,    // 407 - "<target> :<error code> recipients. <abort message>"
    ErrNoSuchService,     // 408 - "<service name> :No such service"
    ErrNoOrigin,          // 409 - ":No origin specified"
    ErrNoRecipient,       // 411 - ":No recipient given (<command>)"
    ErrNoTextToSend,      // 412 - ":No text to send"
    ErrNoTopLevel,        // 413 - "<mask> :No toplevel domain specified"
    ErrWildTopLevel,      // 414 - "<mask> :Wildcard in toplevel domain"
    ErrBadMask,           // 415 - "<mask> :Bad Server/host mask"
    ErrUnknownCommand,    // 421 - "<command> :Unknown command"
    ErrNoMotd,            // 422 - ":MOTD File is missing"
    ErrNoAdminInfo,       // 423 - "<server> :No administrative info available"
    ErrFileError,         // 424 - ":File error doing <file op> on <file>"
    ErrNoNicknameGiven,   // 431 - ":No nickname given"
    ErrErroneusNickname,  // 432 - "<nick> :Erroneous nickname"
    ErrNicknameInUse,     // 433 - "<nick> :Nickname is already in use"
    ErrNickCollision,     // 436 - "<nick> :Nickname collision KILL from <user>@<host>"
    ErrUnavailResource,   // 437 - "<nick/channel> :Nick/channel is temporarily unavailable"
    ErrUserNotInChannel,  // 441 - "<nick> <channel> :They aren't on that channel"
    ErrNotOnChannel,      // 442 - "<channel> :You're not on that channel"
    ErrUserOnChannel,     // 443 - "<user> <channel> :is already on channel"
    ErrNoLogin,           // 444 - "<user> :User not logged in"
    ErrSummonDisabled,    // 445 - ":SUMMON has been disabled"
    ErrUsersDisabled,     // 446 - ":USERS has been disabled"
    ErrNotRegistered,     // 451 - ":You have not registered"
    ErrNeedMoreParams,    // 461 - "<command> :Not enough parameters"
    ErrAlreadyRegistred,  // 462 - ":Unauthorized command (already registered)"
    ErrNoPermForHost,     // 463 - ":Your host isn't among the privileged"
    ErrPasswdMismatch,    // 464 - ":Password incorrect"
    ErrYoureBannedCreep,  // 465 - ":You are banned from this server"
    ErrYouWillBeBanned,   // 466
    ErrKeySet,            // 467 - "<channel> :Channel key already set"
    ErrChannelIsFull,     // 471 - "<channel> :Cannot join channel (+l)"
    ErrUnknownMode,       // 472 - "<char> :is unknown mode char to me for <channel>"
    ErrInviteOnlyChan,    // 473 - "<channel> :Cannot join channel (+i)"
    ErrBannedFromChan,    // 474 - "<channel> :Cannot join channel (+b)"
    ErrBadChannelKey,     // 475 - "<channel> :Cannot join channel (+k)"
    ErrBadChanMask,       // 476 - "<channel> :Bad Channel Mask"
    ErrNoChanModes,       // 477 - "<channel> :Channel doesn't support modes"
    ErrBanListFull,       // 478 - "<channel> <char> :Channel list is full"
    ErrNoPrivileges,      // 481 - ":Permission Denied- You're not an IRC operator"
    ErrChanOPrivsNeeded,  // 482 - "<channel> :You're not channel operator"
    ErrCantKillServer,    // 483 - ":You can't kill a server!"
    ErrRestricted,        // 484 - ":Your connection is restricted!"
    ErrUniqOpPrivsNeeded, // 485 - ":You're not the original channel operator"
    ErrNoOperHost,        // 491 - ":No O-lines for your host"
    ErrUModeUnknownFlag,  // 501 - ":Unknown MODE flag"
    ErrUsersDontMatch,    // 502 - ":Cannot change mode for other users"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_command_type_pass() {
        assert_eq!(
            CommandType::Pass,
            Command::Pass {
                password: "mypass".to_string()
            }
            .to_command_type(),
        );
    }

    #[test]
    fn to_command_type_nick() {
        assert_eq!(
            CommandType::Nick,
            Command::Nick {
                nickname: "me".to_string(),
                hopcount: None
            }
            .to_command_type(),
        );
    }

    #[test]
    fn to_command_type_user() {
        assert_eq!(
            CommandType::User,
            Command::User {
                username: "pjohnson".to_string(),
                hostname: "local".to_string(),
                servername: "remote".to_string(),
                realname: "Potato Johnson".to_string(),
            }
            .to_command_type(),
        );
    }

    #[test]
    fn to_command_type_ping() {
        assert_eq!(
            CommandType::Ping,
            Command::Ping {
                server1: "myserver".to_string(),
                server2: None
            }
            .to_command_type(),
        );
    }

    #[test]
    fn to_command_type_pong() {
        assert_eq!(
            CommandType::Pong,
            Command::Pong {
                server1: "myclient".to_string(),
                server2: None
            }
            .to_command_type(),
        );
    }

    #[test]
    fn command_to_raw_pass() {
        assert_eq!(
            "PASS mysecretpass",
            command_to_raw(Command::Pass {
                password: "mysecretpass".to_string(),
            }),
        );
    }

    #[test]
    fn command_to_raw_nick() {
        assert_eq!(
            "NICK potato",
            command_to_raw(Command::Nick {
                nickname: "potato".to_string(),
                hopcount: None,
            }),
        );
        assert_eq!(
            "NICK carrot 5",
            command_to_raw(Command::Nick {
                nickname: "carrot".to_string(),
                hopcount: Some(5),
            }),
        );
    }

    #[test]
    fn command_to_raw_user() {
        assert_eq!(
            "USER ab cd ef :gh ij",
            command_to_raw(Command::User {
                username: "ab".to_string(),
                hostname: "cd".to_string(),
                servername: "ef".to_string(),
                realname: "gh ij".to_string(),
            }),
        );
    }

    #[test]
    fn command_to_raw_ping() {
        assert_eq!(
            "PING myserver",
            command_to_raw(Command::Ping {
                server1: "myserver".to_string(),
                server2: None,
            }),
        );
        assert_eq!(
            "PING myserver myotherserver",
            command_to_raw(Command::Ping {
                server1: "myserver".to_string(),
                server2: Some("myotherserver".to_string()),
            }),
        );
    }

    #[test]
    fn command_to_raw_pong() {
        assert_eq!(
            "PONG myclient",
            command_to_raw(Command::Pong {
                server1: "myclient".to_string(),
                server2: None,
            }),
        );
        assert_eq!(
            "PONG myclient myotherclient",
            command_to_raw(Command::Pong {
                server1: "myclient".to_string(),
                server2: Some("myotherclient".to_string()),
            }),
        );
    }

    #[test]
    fn raw_to_command_pass() {
        let command = raw_to_command("PASS mysecretpass");
        if let Some(Command::Pass { password }) = command {
            assert_eq!("mysecretpass", password);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn raw_to_command_nick() {
        let command = raw_to_command("NICK somebody");
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("somebody", nickname);
            assert_eq!(None, hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command("NICK anybody 5");
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("anybody", nickname);
            assert_eq!(Some(5), hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command("NICK anybody potato");
        if let Some(Command::Nick { nickname, hopcount }) = command {
            assert_eq!("anybody", nickname);
            assert_eq!(None, hopcount);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn raw_to_command_user() {
        assert!(raw_to_command("USER pjohnson local remote").is_none());
        assert!(raw_to_command("USER pjohnson local remote realname").is_none());
        assert!(raw_to_command("USER pjohnson local :remote realname").is_none());

        let command = raw_to_command("USER pjohnson local remote :Potato Johnson");
        if let Some(Command::User {
            username,
            hostname,
            servername,
            realname,
        }) = command
        {
            assert_eq!("pjohnson", username);
            assert_eq!("local", hostname);
            assert_eq!("remote", servername);
            assert_eq!("Potato Johnson", realname);
        } else {
            panic!("Wrong type: {:?}", command);
        }
    }

    #[test]
    fn raw_to_command_ping() {
        let command = raw_to_command("PING myserver");
        if let Some(Command::Ping { server1, server2 }) = command {
            assert_eq!("myserver", server1);
            assert!(server2.is_none());
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command("PING myserver myotherserver");
        if let Some(Command::Ping { server1, server2 }) = command {
            assert_eq!("myserver", server1);
            assert_eq!(Some("myotherserver".to_string()), server2);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        assert!(raw_to_command("PING").is_none());
        assert!(raw_to_command("PING a b c").is_none());
    }

    #[test]
    fn raw_to_command_pong() {
        let command = raw_to_command("PONG myclient");
        if let Some(Command::Pong { server1, server2 }) = command {
            assert_eq!("myclient", server1);
            assert!(server2.is_none());
        } else {
            panic!("Wrong type: {:?}", command);
        }

        let command = raw_to_command("PONG myclient myotherclient");
        if let Some(Command::Pong { server1, server2 }) = command {
            assert_eq!("myclient", server1);
            assert_eq!(Some("myotherclient".to_string()), server2);
        } else {
            panic!("Wrong type: {:?}", command);
        }

        assert!(raw_to_command("PONG").is_none());
        assert!(raw_to_command("PONG a b c").is_none());
    }

    #[test]
    fn split_server_name_with_name() {
        let mut command =
            ":irc.example.net 001 foo :Welcome to the Internet Relay Network".to_string();
        let result = split_server_name(&mut command);

        assert_eq!("001 foo :Welcome to the Internet Relay Network", command);
        assert_eq!(Some("irc.example.net".to_string()), result);
    }

    #[test]
    fn split_server_name_no_name() {
        let mut command = "001 foo :Welcome to the Internet Relay Network".to_string();
        let result = split_server_name(&mut command);

        assert_eq!("001 foo :Welcome to the Internet Relay Network", command);
        assert_eq!(None, result);
    }

    #[test]
    fn split_server_name_missing_colon() {
        let mut command =
            "irc.example.net 001 foo :Welcome to the Internet Relay Network".to_string();
        let result = split_server_name(&mut command);

        assert_eq!(
            "irc.example.net 001 foo :Welcome to the Internet Relay Network",
            command
        );
        assert_eq!(None, result);
    }

    #[test]
    fn split_server_name_server_only() {
        let mut command = ":irc.example.net".to_string();
        let result = split_server_name(&mut command);

        assert_eq!(":irc.example.net", command);
        assert_eq!(None, result);
    }

    #[test]
    fn split_server_name_trailing_space() {
        let mut command = ":irc.example.net ".to_string();
        let result = split_server_name(&mut command);

        assert_eq!("", command);
        assert_eq!(Some("irc.example.net".to_string()), result);
    }
}
