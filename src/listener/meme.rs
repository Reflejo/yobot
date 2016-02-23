extern crate slack;
extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;

use regex::Regex;
use rustc_serialize::json::Json;
use listener::{MessageListener, Message};
use std::io::Read;


/// Listens for the word `meme` and takes two arguments, one for the meme kind and
/// another for the two (or one) lines text separated by `|`.
///
/// Example: @yobot meme sohot dat bot
pub struct MemeListener {
    regex: Regex,
	templates: Vec<String>
}

impl MemeListener {
    pub fn new() -> MemeListener {
        MemeListener {
            regex: Regex::new(r"meme (\S+) ([\S, ]+)").unwrap(),
			templates: MemeListener::templates()
        }
    }

	// Meme generator logic

	fn templates() -> Vec<String> {
		let client = Client::new();
		let mut response = match client.get("http://memegen.link/templates").send() {
			Ok(response) => response,
			Err(_request) => unreachable!(), // Uncaught condition will have failed first
		};
		
		let mut options: Vec<String> = Vec::new();
		let mut body = String::new();
		let _ = response.read_to_string(&mut body);

		if let Ok(json) = Json::from_str(&body) {
			for (_, value) in json.as_object().unwrap().iter() {
				let parts: Vec<&str> = value.as_string().unwrap_or("")
					.split("/").collect();

				if let Some(option) = parts.last() {
					options.push(String::from(*option));
				}
			}
		}

		options
	}
}

impl MessageListener for MemeListener {
    fn help(&self) -> String {
		format!("`meme`: Post memes, _Usage: meme <type> <line1>|<line2>_.\n \
				\t\tOptions are: *{}*", self.templates.join(", "))
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
		let captures = self.get_captures(message).unwrap();
		let (kind, text) = (String::from(captures.at(1).unwrap()), captures.at(2).unwrap());
		if !self.templates.contains(&kind) {
			let msg = format!("Options are: *{}*", self.templates.join(", "));
        	let _ = cli.send_message(&message.channel, &msg);
			return
		}

		let lines: Vec<&str> = text.split('|').collect();
		let (first, second) = if lines.len() > 1 { (lines[0], lines[1]) } else { (lines[0], "") };
		let url = format!("http://memegen.link/{}/{}/{}.jpg", kind, first, second)
			.replace(" ", "%20");

        let _ = cli.send_message(&message.channel, &url);
    }
}
