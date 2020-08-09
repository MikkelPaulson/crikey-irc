use crate::connection;
use std::collections::HashMap;

pub struct Dispatcher<L>
where
    L: Fn(connection::Command),
{
    command_listeners: HashMap<connection::CommandType, Vec<L>>,
    reply_listeners: HashMap<connection::ReplyType, Vec<L>>,
}

impl<L> Dispatcher<L>
where
    L: Fn(connection::Command),
{
    fn register_command_listener(&mut self, command_type: connection::CommandType, listener: L) {
        self.command_listeners
            .entry(command_type)
            .or_insert(Vec::new())
            .push(listener);
    }

    fn register_reply_listener(&mut self, reply_type: connection::ReplyType, listener: L) {
        self.reply_listeners
            .entry(reply_type)
            .or_insert(Vec::new())
            .push(listener);
    }

    fn handle_command() {}

    fn handle_reply() {}
}
