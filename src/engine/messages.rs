use std::string::String;

pub struct MessageList {
    messages: Vec<Message>,
}

pub enum MessageType {
    Info,
    Error,
}

pub struct Message {
    text: String,
    ty: MessageType,
}

impl MessageList {

    pub fn new() -> MessageList {
        MessageList { messages: vec![] }
    }

    pub fn info(&mut self, text: &str) {
        self.messages.push(Message { text: String::from_str(text), ty: MessageType::Info });
    }

    pub fn error(&mut self, text: &str) {
        self.messages.push(Message { text: String::from_str(text), ty: MessageType::Error });
    }

}
