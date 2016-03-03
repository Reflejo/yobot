//! Contains the MessageListener trait implementations.

use regex::Captures;
use regex::Regex;
use slack::{User, RtmClient};

pub mod echo;
pub mod meme;
pub mod slackbot;
pub mod profound;

pub struct Message {
    pub user: User,
    pub text: String,
    pub is_addressed: bool,
    pub channel: String,
}

/// Implementing a MessageListener enables responding to Messages. There
/// are currently very few requirements to creating a handler. The
/// [`handle`](#method.handle) function receives a Message and a RtmClient.
/// The listener is responsible for testing whether it's interested in replying by
/// defining a regular expression on [`re`](#method.re)
/// Optionally call `cli.send_message` to send a response.
///
/// # Example
///
/// A simple echo handler might look something like the following:
///
/// ```rust
/// # extern crate slack;
/// # extern crate regex;
/// # fn main() {
/// use regex::Regex;
/// use yobot::listener::{MessageListener, Message};
///
/// pub struct EchoListener {
///     regex: Regex
/// }
///
/// impl EchoListener {
///     pub fn new() -> EchoListener {
///         EchoListener {
///             regex: Regex::new(r".").unwrap()
///         }
///     }
/// }
///
/// impl MessageListener for EchoListener {
///     fn help(&self) -> String {
///         String::from("echo")
///     }
///
///     fn re(&self) -> &Regex {
///         &self.regex
///     }
///
///     fn handle(&self, message: &Message, cli: &slack::RtmClient) {
///         let _ = cli.send_message(&message.channel, &message.text);
///     }
/// }
/// # }
/// ```
pub trait MessageListener {
    fn help(&self) -> String;
    fn handle(&self, message: &Message, cli: &RtmClient);
    fn re(&self) -> &Regex;

    /// Uses re() to test whether the handler should process this message.
    fn can_handle(&self, msg: &Message) -> bool {
        self.re().is_match(&msg.text) && (!self.only_when_addressed() || msg.is_addressed)
    }

    /// Uses re() to get capturing groups from a message
    fn get_captures<'a>(&self, msg: &'a Message) -> Option<Captures<'a>> {
        self.re().captures(&msg.text)
    }

    /// When true, this filter will only apply when the bot is addressed by name
    fn only_when_addressed(&self) -> bool {
        true
    }
}
