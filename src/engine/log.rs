use std::string::String;
use std::slice::Iter;
use std::cell::RefCell;

thread_local!(pub static LOG: RefCell<MessageList> = RefCell::new(MessageList::new()));

pub fn info(text: &str) {
    LOG.with(|w| w.borrow_mut().info(text));
}

pub fn error(text: &str) {
    LOG.with(|w| w.borrow_mut().error(text));
}

pub enum MessageType {
    Info,
    Error,
}

pub struct Message {
    text: String,
    ty: MessageType,
}

impl Message {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn ty(&self) -> &MessageType {
        &self.ty
    }
}

pub struct MessageList {
    messages: Vec<Message>,
}

impl MessageList {

    pub fn new() -> MessageList {
        MessageList { messages: vec![] }
    }

    pub fn info(&mut self, text: &str) {
        self.messages.insert(0, Message { text: text.to_string(), ty: MessageType::Info });
    }

    pub fn error(&mut self, text: &str) {
        self.messages.insert(0, Message { text: text.to_string(), ty: MessageType::Error });
    }

    pub fn items(&self) -> Iter<Message> {
        self.messages.iter()
    }

}
