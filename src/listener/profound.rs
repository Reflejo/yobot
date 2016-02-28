extern crate hyper;
extern crate slack;

use hyper::Client;
use listener::{MessageListener, Message};
use regex::Regex;


/// Listens for the words `be profound` and posts a random profound programmer link
///
/// Example @yobot be profound
pub struct ProfoundListener {
    regex: Regex,
}

impl ProfoundListener {
    pub fn new() -> ProfoundListener {
        ProfoundListener {
            regex: Regex::new(r"be profound").unwrap(),
        }
    }
}

impl MessageListener for ProfoundListener {
    fn help(&self) -> String {
        "`be profound`: Post a random profound programmer image".to_owned()
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let client = Client::new();
        let url = "http://theprofoundprogrammer.com/random";
        let text = match client.get(url).send() {
            Ok(response) => response.url.serialize(),
            Err(_) => "Maybe next time".to_owned(),
        };

        let _ = cli.send_message(&message.channel, &text);
    }
}
