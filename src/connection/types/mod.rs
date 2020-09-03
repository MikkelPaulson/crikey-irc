pub use self::channel::{Channel, ChannelID, ChannelKey, ChannelName, ChannelType};
pub use self::host::{Host, Hostname, IpAddr, Ipv4Addr, Ipv6Addr, Servername};
pub use self::keyword_list::KeywordList;
pub use self::msg_target::{Recipient, Sender};
pub use self::nickname::Nickname;
pub use self::stats_query::StatsQuery;
pub use self::target_mask::{HostMask, ServerMask, TargetMask};
pub use self::user::User;

mod channel;
mod host;
mod keyword_list;
mod msg_target;
mod nickname;
mod stats_query;
mod target_mask;
mod user;

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct ParseError(&'static str);

impl ParseError {
    pub fn new(struct_name: &'static str) -> Self {
        ParseError(struct_name)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse component: {}", self)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
