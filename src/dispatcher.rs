use crate::connection;
use std::collections::HashMap;

pub trait Dispatch {
    fn register_command_listener(
        &mut self,
        command_type: connection::CommandType,
        command_listener: &'static dyn Fn(&connection::Command),
    );

    fn register_reply_listener(
        &mut self,
        reply_listener: &'static dyn Fn(&connection::ReplyType, &String) -> bool,
    );

    fn handle_command(&mut self, command: connection::Command);

    fn handle_reply(&mut self, reply_type: connection::ReplyType, message: String);
}

pub struct Dispatcher {
    command_listeners:
        HashMap<connection::CommandType, Vec<Box<dyn 'static + Fn(&connection::Command)>>>,
    reply_listeners: Vec<Box<dyn 'static + Fn(&connection::ReplyType, &String) -> bool>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            command_listeners: HashMap::new(),
            reply_listeners: Vec::new(),
        }
    }
}

impl Dispatch for Dispatcher {
    fn register_command_listener(
        &mut self,
        command_type: connection::CommandType,
        command_listener: &'static dyn Fn(&connection::Command),
    ) {
        self.command_listeners
            .entry(command_type)
            .or_insert(Vec::new())
            .push(Box::new(command_listener));
    }

    fn register_reply_listener(
        &mut self,
        reply_listener: &'static dyn Fn(&connection::ReplyType, &String) -> bool,
    ) {
        self.reply_listeners.push(Box::new(reply_listener));
    }

    fn handle_command(&mut self, command: connection::Command) {
        let command_type = command.to_command_type();

        for command_listener in self.command_listeners.entry(command_type).or_default() {
            command_listener(&command);
        }
    }

    fn handle_reply(&mut self, reply_type: connection::ReplyType, message: String) {
        let mut i = 0;

        while i < self.reply_listeners.len() {
            let listener = &self.reply_listeners[i];
            if listener(&reply_type, &message) {
                i += 1;
            } else {
                let _ = self.reply_listeners.remove(i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Test passed: Pass { password: \"abc\" }")]
    fn command_listener_match() {
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_command_listener(connection::CommandType::Pass, &|_| {});
        dispatcher.register_command_listener(connection::CommandType::Pass, &|command| {
            panic!("Test passed: {:?}", command);
        });
        dispatcher.handle_command(connection::Command::Pass {
            password: "abc".to_string(),
        });
    }

    #[test]
    fn command_listener_no_match() {
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_command_listener(connection::CommandType::Nick, &|command| {
            panic!("Test failed: {:?}", command);
        });
        dispatcher.handle_command(connection::Command::Pass {
            password: "abc".to_string(),
        });
    }

    #[test]
    #[should_panic(expected = "Test passed: ErrYoureBannedCreep \"You\\\'re banned, creep!\"")]
    fn reply_listener_runs() {
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_reply_listener(&|_, _| true);
        dispatcher.register_reply_listener(&|reply_type, message| {
            panic!("Test passed: {:?} {:?}", reply_type, message);
        });
        dispatcher.handle_reply(
            connection::ReplyType::ErrYoureBannedCreep,
            "You're banned, creep!".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "Test passed: RplYourHost \"Message 2\"")]
    fn reply_listener_persists() {
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_reply_listener(&|reply_type, message| {
            if message == "Message 2" {
                panic!("Test passed: {:?} {:?}", reply_type, message)
            }
            true // true to persist the listener between invocations
        });

        dispatcher.handle_reply(connection::ReplyType::RplWelcome, "Message 1".to_string());
        dispatcher.handle_reply(connection::ReplyType::RplYourHost, "Message 2".to_string());
    }

    #[test]
    fn reply_listener_unregisters_itself() {
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_reply_listener(&|reply_type, message| {
            if message == "Message 2" {
                panic!("Test failed: {:?} {:?}", reply_type, message)
            }
            false // false to unregister the listener after the first run
        });

        dispatcher.handle_reply(connection::ReplyType::RplWelcome, "Message 1".to_string());
        dispatcher.handle_reply(connection::ReplyType::RplYourHost, "Message 2".to_string());
    }
}
