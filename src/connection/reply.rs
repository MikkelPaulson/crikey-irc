pub fn from_raw(raw_reply: &str) -> Option<(ReplyType, String)> {
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
    fn from_raw_welcome() {
        if let Some((reply_type, reply_body)) =
            from_raw("001 :Welcome to the Internet Relay Network nick!~username@host")
        {
            assert_eq!(ReplyType::RplWelcome, reply_type);
            assert_eq!(
                ":Welcome to the Internet Relay Network nick!~username@host",
                reply_body
            );
        } else {
            panic!("Wrong type: None");
        }
    }
}
