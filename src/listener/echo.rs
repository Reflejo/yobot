extern crate slack;

use regex::Regex;
use listener::{MessageListener, Message};

pub struct EchoListener {
    regex: Regex
}

impl EchoListener {
    pub fn new() -> EchoListener {
        EchoListener {
            regex: Regex::new(r".").unwrap()
        }
    }
}

impl MessageListener for EchoListener {
    fn help(&self) -> &str {
        "echo: Just type anything"
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn only_when_addressed(&self) -> bool {
        false
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let _ = cli.send_message(&message.channel, &message.text);
    }
}
