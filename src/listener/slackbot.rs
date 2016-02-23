extern crate slack;
extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;

use regex::Regex;
use listener::{MessageListener, Message};
use std::env;


/// Listens for the phrase `make slackbot say <message> in <channel>` and makes
/// @slackbot say the phrase on the given channel.
///
/// Example: @yobot make slackbot say Hello in #ios
pub struct SlackbotListener {
    regex: Regex,
    token: String
}

impl SlackbotListener {
    pub fn new() -> SlackbotListener {
        SlackbotListener {
            regex: Regex::new(r"make slackbot say (.+) in (<?#?\S+>?)$").unwrap(),
            token: env::var("SLACK_TOKEN").unwrap_or("".to_string())
        }
    }
}

impl MessageListener for SlackbotListener {
    fn help(&self) -> String {
		format!("`make slackbot say`: Make slackbot talk in other channels. \
                _Usage: make slackbot say <message> in <channel>_")
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn handle(&self, message: &Message, _cli: &slack::RtmClient) {
		let captures = self.get_captures(message).unwrap();
		let (message, mut channel) = (captures.at(1).unwrap(), captures.at(2).unwrap().to_string());

		let client = Client::new();
        if channel.starts_with("<#") && channel.len() > 2 {
            channel = channel[2..channel.len() - 1].to_string();
        } else {
            channel = format!("%23{}", channel);
        }

        debug!("Making slackbot say {} in {}", message, channel);
        let url = format!("https://lyft.slack.com/services/hooks/slackbot\
                          ?token={}&channel={}", self.token, channel);
		let _ = client.post(&url).body(message).send();
    }

    fn only_when_addressed(&self) -> bool {
        false
    }
}
