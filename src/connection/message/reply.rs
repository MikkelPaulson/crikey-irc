use super::{MessageParams, ParseError};
use std::result::Result;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Reply {
    pub reply_type: ReplyType,
    pub params: MessageParams,
}

impl FromStr for Reply {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() < 3 || !raw.is_char_boundary(3) {
            return Err(ParseError::new("Reply"));
        }

        if let Ok(reply_type) = raw[..3].parse() {
            Ok(Reply {
                reply_type,
                params: raw[3..].parse()?,
            })
        } else {
            Err(ParseError::new("Reply"))
        }
    }
}

impl From<Reply> for String {
    fn from(reply: Reply) -> String {
        let mut reply_text = String::from(reply.reply_type);
        reply_text.push(' ');
        reply_text.push_str(&String::from(reply.params));
        reply_text
    }
}

#[derive(PartialEq, Debug)]
pub enum ReplyType {
    PrvWelcome,           // 001 - "Welcome to the Internet Relay Network
    PrvYourHost,          // 002 - "Your host is <servername>, running version <ver>"
    PrvCreated,           // 003 - "This server was created <date>"
    PrvMyInfo,            // 004 - "<servername> <version> <available user modes>
    PrvBounce,            // 005 - "Try server <server name>, port <port number>"
    PrvUnknown(u16),      // 0xx
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
    RplUnknown(u16),      // [23]xx
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
    ErrUnknown(u16),      // [45]xx
}

#[allow(overlapping_patterns)]
impl FromStr for ReplyType {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() != 3 || !raw.is_ascii() {
            Err(ParseError::new("ReplyType"))
        } else {
            let raw_int: u16 = raw.parse().map_err(|_| ParseError::new("ReplyType"))?;
            Ok(match raw_int {
                001 => ReplyType::PrvWelcome,
                002 => ReplyType::PrvYourHost,
                003 => ReplyType::PrvCreated,
                004 => ReplyType::PrvMyInfo,
                005 => ReplyType::PrvBounce,
                200 => ReplyType::RplTraceLink,
                201 => ReplyType::RplTraceConnecting,
                202 => ReplyType::RplTraceHandshake,
                203 => ReplyType::RplTraceUnknown,
                204 => ReplyType::RplTraceOperator,
                205 => ReplyType::RplTraceUser,
                206 => ReplyType::RplTraceServer,
                207 => ReplyType::RplTraceService,
                208 => ReplyType::RplTraceNewType,
                209 => ReplyType::RplTraceClass,
                210 => ReplyType::RplTraceReconnect,
                211 => ReplyType::RplStatsLinkInfo,
                212 => ReplyType::RplStatsCommands,
                219 => ReplyType::RplEndOfStats,
                221 => ReplyType::RplUModeIs,
                234 => ReplyType::RplServList,
                235 => ReplyType::RplServListEnd,
                242 => ReplyType::RplStatsUptime,
                243 => ReplyType::RplStatsOLine,
                251 => ReplyType::RplLUserClient,
                252 => ReplyType::RplLUserOp,
                253 => ReplyType::RplLUserUnknown,
                254 => ReplyType::RplLUserChannels,
                255 => ReplyType::RplLUserMe,
                256 => ReplyType::RplAdminMe,
                257 => ReplyType::RplAdminLoc1,
                258 => ReplyType::RplAdminLoc2,
                259 => ReplyType::RplAdminEmail,
                261 => ReplyType::RplTraceLog,
                262 => ReplyType::RplTraceEnd,
                263 => ReplyType::RplTryAgain,
                301 => ReplyType::RplAway,
                302 => ReplyType::RplUserHost,
                303 => ReplyType::RplIsOn,
                305 => ReplyType::RplUnAway,
                306 => ReplyType::RplNowAway,
                311 => ReplyType::RplWhoIsUser,
                312 => ReplyType::RplWhoIsServer,
                313 => ReplyType::RplWhoIsOperator,
                314 => ReplyType::RplWhoWasUser,
                315 => ReplyType::RplEndOfWho,
                317 => ReplyType::RplWhoIsIdle,
                318 => ReplyType::RplEndOfWhoIs,
                319 => ReplyType::RplWhoIsChannels,
                321 => ReplyType::RplListStart,
                322 => ReplyType::RplList,
                323 => ReplyType::RplListEnd,
                324 => ReplyType::RplChannelModeIs,
                325 => ReplyType::RplUniqOpIs,
                331 => ReplyType::RplNoTopic,
                332 => ReplyType::RplTopic,
                341 => ReplyType::RplInviting,
                342 => ReplyType::RplSummoning,
                346 => ReplyType::RplInviteList,
                347 => ReplyType::RplEndOfInviteList,
                348 => ReplyType::RplExceptList,
                349 => ReplyType::RplEndOfExceptList,
                351 => ReplyType::RplVersion,
                352 => ReplyType::RplWhoReply,
                353 => ReplyType::RplNamReply,
                364 => ReplyType::RplLinks,
                365 => ReplyType::RplEndOfLinks,
                366 => ReplyType::RplEndOfNames,
                367 => ReplyType::RplBanList,
                368 => ReplyType::RplEndOfBanList,
                369 => ReplyType::RplEndOfWhoWas,
                371 => ReplyType::RplInfo,
                372 => ReplyType::RplMotd,
                374 => ReplyType::RplEndOfInfo,
                375 => ReplyType::RplMotdStart,
                376 => ReplyType::RplEndOfMotd,
                381 => ReplyType::RplYoureOper,
                382 => ReplyType::RplRehashing,
                383 => ReplyType::RplYoureService,
                391 => ReplyType::RplTime,
                392 => ReplyType::RplUsersStart,
                393 => ReplyType::RplUsers,
                394 => ReplyType::RplEndOfUsers,
                395 => ReplyType::RplNoUsers,
                401 => ReplyType::ErrNoSuchNick,
                402 => ReplyType::ErrNoSuchServer,
                403 => ReplyType::ErrNoSuchChannel,
                404 => ReplyType::ErrCannotSendToChan,
                405 => ReplyType::ErrTooManyChannels,
                406 => ReplyType::ErrWasNoSuchNick,
                407 => ReplyType::ErrTooManyTargets,
                408 => ReplyType::ErrNoSuchService,
                409 => ReplyType::ErrNoOrigin,
                411 => ReplyType::ErrNoRecipient,
                412 => ReplyType::ErrNoTextToSend,
                413 => ReplyType::ErrNoTopLevel,
                414 => ReplyType::ErrWildTopLevel,
                415 => ReplyType::ErrBadMask,
                421 => ReplyType::ErrUnknownCommand,
                422 => ReplyType::ErrNoMotd,
                423 => ReplyType::ErrNoAdminInfo,
                424 => ReplyType::ErrFileError,
                431 => ReplyType::ErrNoNicknameGiven,
                432 => ReplyType::ErrErroneusNickname,
                433 => ReplyType::ErrNicknameInUse,
                436 => ReplyType::ErrNickCollision,
                437 => ReplyType::ErrUnavailResource,
                441 => ReplyType::ErrUserNotInChannel,
                442 => ReplyType::ErrNotOnChannel,
                443 => ReplyType::ErrUserOnChannel,
                444 => ReplyType::ErrNoLogin,
                445 => ReplyType::ErrSummonDisabled,
                446 => ReplyType::ErrUsersDisabled,
                451 => ReplyType::ErrNotRegistered,
                461 => ReplyType::ErrNeedMoreParams,
                462 => ReplyType::ErrAlreadyRegistred,
                463 => ReplyType::ErrNoPermForHost,
                464 => ReplyType::ErrPasswdMismatch,
                465 => ReplyType::ErrYoureBannedCreep,
                466 => ReplyType::ErrYouWillBeBanned,
                467 => ReplyType::ErrKeySet,
                471 => ReplyType::ErrChannelIsFull,
                472 => ReplyType::ErrUnknownMode,
                473 => ReplyType::ErrInviteOnlyChan,
                474 => ReplyType::ErrBannedFromChan,
                475 => ReplyType::ErrBadChannelKey,
                476 => ReplyType::ErrBadChanMask,
                477 => ReplyType::ErrNoChanModes,
                478 => ReplyType::ErrBanListFull,
                481 => ReplyType::ErrNoPrivileges,
                482 => ReplyType::ErrChanOPrivsNeeded,
                483 => ReplyType::ErrCantKillServer,
                484 => ReplyType::ErrRestricted,
                485 => ReplyType::ErrUniqOpPrivsNeeded,
                491 => ReplyType::ErrNoOperHost,
                501 => ReplyType::ErrUModeUnknownFlag,
                502 => ReplyType::ErrUsersDontMatch,
                0..=99 => ReplyType::PrvUnknown(raw_int),
                200..=399 => ReplyType::RplUnknown(raw_int),
                400..=599 => ReplyType::ErrUnknown(raw_int),
                _ => return Err(ParseError::new("ReplyType")),
            })
        }
    }
}

impl From<ReplyType> for String {
    fn from(reply_type: ReplyType) -> String {
        format!(
            "{:0>3}",
            match reply_type {
                ReplyType::PrvWelcome => 001,
                ReplyType::PrvYourHost => 002,
                ReplyType::PrvCreated => 003,
                ReplyType::PrvMyInfo => 004,
                ReplyType::PrvBounce => 005,
                ReplyType::PrvUnknown(code) => code,
                ReplyType::RplUserHost => 302,
                ReplyType::RplIsOn => 303,
                ReplyType::RplAway => 301,
                ReplyType::RplUnAway => 305,
                ReplyType::RplNowAway => 306,
                ReplyType::RplWhoIsUser => 311,
                ReplyType::RplWhoIsServer => 312,
                ReplyType::RplWhoIsOperator => 313,
                ReplyType::RplWhoIsIdle => 317,
                ReplyType::RplEndOfWhoIs => 318,
                ReplyType::RplWhoIsChannels => 319,
                ReplyType::RplWhoWasUser => 314,
                ReplyType::RplEndOfWhoWas => 369,
                ReplyType::RplListStart => 321,
                ReplyType::RplList => 322,
                ReplyType::RplListEnd => 323,
                ReplyType::RplUniqOpIs => 325,
                ReplyType::RplChannelModeIs => 324,
                ReplyType::RplNoTopic => 331,
                ReplyType::RplTopic => 332,
                ReplyType::RplInviting => 341,
                ReplyType::RplSummoning => 342,
                ReplyType::RplInviteList => 346,
                ReplyType::RplEndOfInviteList => 347,
                ReplyType::RplExceptList => 348,
                ReplyType::RplEndOfExceptList => 349,
                ReplyType::RplVersion => 351,
                ReplyType::RplWhoReply => 352,
                ReplyType::RplEndOfWho => 315,
                ReplyType::RplNamReply => 353,
                ReplyType::RplEndOfNames => 366,
                ReplyType::RplLinks => 364,
                ReplyType::RplEndOfLinks => 365,
                ReplyType::RplBanList => 367,
                ReplyType::RplEndOfBanList => 368,
                ReplyType::RplInfo => 371,
                ReplyType::RplEndOfInfo => 374,
                ReplyType::RplMotdStart => 375,
                ReplyType::RplMotd => 372,
                ReplyType::RplEndOfMotd => 376,
                ReplyType::RplYoureOper => 381,
                ReplyType::RplRehashing => 382,
                ReplyType::RplYoureService => 383,
                ReplyType::RplTime => 391,
                ReplyType::RplUsersStart => 392,
                ReplyType::RplUsers => 393,
                ReplyType::RplEndOfUsers => 394,
                ReplyType::RplNoUsers => 395,
                ReplyType::RplTraceLink => 200,
                ReplyType::RplTraceConnecting => 201,
                ReplyType::RplTraceHandshake => 202,
                ReplyType::RplTraceUnknown => 203,
                ReplyType::RplTraceOperator => 204,
                ReplyType::RplTraceUser => 205,
                ReplyType::RplTraceServer => 206,
                ReplyType::RplTraceService => 207,
                ReplyType::RplTraceNewType => 208,
                ReplyType::RplTraceClass => 209,
                ReplyType::RplTraceReconnect => 210,
                ReplyType::RplTraceLog => 261,
                ReplyType::RplTraceEnd => 262,
                ReplyType::RplStatsLinkInfo => 211,
                ReplyType::RplStatsCommands => 212,
                ReplyType::RplEndOfStats => 219,
                ReplyType::RplStatsUptime => 242,
                ReplyType::RplStatsOLine => 243,
                ReplyType::RplUModeIs => 221,
                ReplyType::RplServList => 234,
                ReplyType::RplServListEnd => 235,
                ReplyType::RplLUserClient => 251,
                ReplyType::RplLUserOp => 252,
                ReplyType::RplLUserUnknown => 253,
                ReplyType::RplLUserChannels => 254,
                ReplyType::RplLUserMe => 255,
                ReplyType::RplAdminMe => 256,
                ReplyType::RplAdminLoc1 => 257,
                ReplyType::RplAdminLoc2 => 258,
                ReplyType::RplAdminEmail => 259,
                ReplyType::RplTryAgain => 263,
                ReplyType::RplUnknown(code) => code,
                ReplyType::ErrNoSuchNick => 401,
                ReplyType::ErrNoSuchServer => 402,
                ReplyType::ErrNoSuchChannel => 403,
                ReplyType::ErrCannotSendToChan => 404,
                ReplyType::ErrTooManyChannels => 405,
                ReplyType::ErrWasNoSuchNick => 406,
                ReplyType::ErrTooManyTargets => 407,
                ReplyType::ErrNoSuchService => 408,
                ReplyType::ErrNoOrigin => 409,
                ReplyType::ErrNoRecipient => 411,
                ReplyType::ErrNoTextToSend => 412,
                ReplyType::ErrNoTopLevel => 413,
                ReplyType::ErrWildTopLevel => 414,
                ReplyType::ErrBadMask => 415,
                ReplyType::ErrUnknownCommand => 421,
                ReplyType::ErrNoMotd => 422,
                ReplyType::ErrNoAdminInfo => 423,
                ReplyType::ErrFileError => 424,
                ReplyType::ErrNoNicknameGiven => 431,
                ReplyType::ErrErroneusNickname => 432,
                ReplyType::ErrNicknameInUse => 433,
                ReplyType::ErrNickCollision => 436,
                ReplyType::ErrUnavailResource => 437,
                ReplyType::ErrUserNotInChannel => 441,
                ReplyType::ErrNotOnChannel => 442,
                ReplyType::ErrUserOnChannel => 443,
                ReplyType::ErrNoLogin => 444,
                ReplyType::ErrSummonDisabled => 445,
                ReplyType::ErrUsersDisabled => 446,
                ReplyType::ErrNotRegistered => 451,
                ReplyType::ErrNeedMoreParams => 461,
                ReplyType::ErrAlreadyRegistred => 462,
                ReplyType::ErrNoPermForHost => 463,
                ReplyType::ErrPasswdMismatch => 464,
                ReplyType::ErrYoureBannedCreep => 465,
                ReplyType::ErrYouWillBeBanned => 466,
                ReplyType::ErrKeySet => 467,
                ReplyType::ErrChannelIsFull => 471,
                ReplyType::ErrUnknownMode => 472,
                ReplyType::ErrInviteOnlyChan => 473,
                ReplyType::ErrBannedFromChan => 474,
                ReplyType::ErrBadChannelKey => 475,
                ReplyType::ErrBadChanMask => 476,
                ReplyType::ErrNoChanModes => 477,
                ReplyType::ErrBanListFull => 478,
                ReplyType::ErrNoPrivileges => 481,
                ReplyType::ErrChanOPrivsNeeded => 482,
                ReplyType::ErrCantKillServer => 483,
                ReplyType::ErrRestricted => 484,
                ReplyType::ErrUniqOpPrivsNeeded => 485,
                ReplyType::ErrNoOperHost => 491,
                ReplyType::ErrUModeUnknownFlag => 501,
                ReplyType::ErrUsersDontMatch => 502,
                ReplyType::ErrUnknown(code) => code,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_values_in_range() {
        for number in (1..=99).chain(200..=599) {
            let number_formatted = format!("{:0>3}", number);
            assert_eq!(
                number_formatted,
                String::from(
                    number_formatted
                        .parse::<ReplyType>()
                        .expect(&number_formatted)
                )
            );
        }
    }
}
