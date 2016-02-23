//!
//! `SlackHandler` handles all the interactions with the slack API. It connects a 
//! persistent socket to the Real Time API and listens for all the events. Events are 
//! communicated back to users of this method by a closure given on the initializer.
//!
//! # Example
//!
//! ```no_run
//! # fn main() {
//! let mut handler = SlackHandler::new(|message, _| { println!("Yo {}", message); });
//! handler.login_and_run(token);
//! # }
//! ```
extern crate regex;
extern crate slack;

use regex::Regex;
use slack::Event;
use std::collections::HashMap;
use listener::Message;

pub struct SlackHandler<F> {
    user_by_id: HashMap<String, slack::User>,
    event_handler: F,
    addresser: Option<Regex>,
}

impl<F> SlackHandler<F>
    where F: Fn(&Message, &slack::RtmClient)
{
    pub fn new(event_handler: F) -> SlackHandler<F> {
        SlackHandler {
            user_by_id: HashMap::new(),
            event_handler: event_handler,
            addresser: None
        }
    }

    pub fn login_and_run(&mut self, token: String) {
        let mut cli = slack::RtmClient::new(&token);

        info!("Updating users ...");
        match cli.list_users() {
            Ok(users) => {
                for user in users.iter() {
                    self.user_by_id.insert(user.id.clone(), user.clone());
                }
            },

            Err(_) => panic!("Can't update users list :(")
        }

        info!("Logging bot in ...");
        let (client, rx) = cli.login().unwrap();
        if let (Some(bot_id), Some(name)) = (cli.get_id(), cli.get_name()) {
            let addresser_regex = format!(r"^\s*<?@?({}|{}|yo)>?[:,\s]\s*", bot_id, name);
            self.addresser = Regex::new(addresser_regex.as_ref()).ok();
        }

        info!("Connecting RTM API ...");
        if let Err(err) = cli.run(self, client, rx) {
            panic!("Error: {}", err);
        }
    }

    fn parse_message(&self, raw: &str) -> (bool, String) {
        let mut message = raw.clone();
        let mut is_addressed = false;

        if let Some(captures) = self.addresser.as_ref().unwrap().captures(raw) {
            is_addressed = true;

            let prefix_len = captures.at(0).unwrap().len();
            let message_len = message.len();
            unsafe {
                message = message.slice_unchecked(prefix_len, message_len);
            }
        }

        (is_addressed, message.to_string())
    }
}

impl<F> slack::EventHandler for SlackHandler<F>
    where F: Fn(&Message, &slack::RtmClient)
{
    fn on_event(&mut self, cli: &mut slack::RtmClient,
                result: Result<&Event, slack::Error>, _: &str)
    {
        if let Ok(&Event::Message(ref message)) = result {

            if let slack::Message::Standard {user: Some(ref user_id),
                                             text: Some(ref text),
                                             channel: Some(ref channel),
                                             ..} = *message
            {
                let bot_id = cli.get_id().unwrap_or("".to_string());
                if *user_id == bot_id {
                    return
                }

                let (is_addressed, message) = self.parse_message(text);
                let user = self.user_by_id.get(user_id).unwrap();
                let message = Message {
                    channel: channel.clone(),
                    is_addressed: is_addressed,
                    text: message,
                    user: user.clone()
                };
                (self.event_handler)(&message, cli);
            }
        }
    }

    fn on_connect(&mut self, _cli: &mut slack::RtmClient) {
        info!("RTM API connected")
    }

    fn on_ping(&mut self, _cli: &mut slack::RtmClient) {}
    fn on_close(&mut self, _cli: &mut slack::RtmClient) {}
}
