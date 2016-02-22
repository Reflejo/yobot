extern crate slack;

use regex::Regex;
use listener::{MessageListener, Message};

pub struct MemeListener {
    regex: Regex
}

impl MemeListener {
    pub fn new() -> MemeListener {
        MemeListener {
            regex: Regex::new(r"meme").unwrap()
        }
    }
}

impl MessageListener for MemeListener {
    fn help(&self) -> &str {
        "meme: Not yet"
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let _ = cli.send_message(&message.channel, "not yet");
    }
}

