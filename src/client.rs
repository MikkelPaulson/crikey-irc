use std::cell::RefCell;
use std::rc::Rc;
use crate::connection;
use crate::dispatcher;

pub struct Client<'a> {
    connection: Rc<RefCell<dyn 'a + connection::Connect>>,
    dispatcher: Rc<RefCell<dyn 'a + dispatcher::Dispatch>>,
}

impl<'a> Client<'a> {
    pub fn new(
        connection: Rc<RefCell<connection::Connection<'a>>>,
        dispatcher: Rc<RefCell<dyn 'a + dispatcher::Dispatch>>
    ) -> Client<'a> {
        Client {
            connection,
            dispatcher
        }
    }

    fn pong(&self, command: &connection::Command) {
        if let connection::Command::Ping { server1, .. } = command {
            self.connection.borrow_mut().send_command(
                connection::Command::Pong {
                    server1: "Me".to_string(),
                    server2: Some(server1.to_string())
                }
            ).ok();
        }
    }
}