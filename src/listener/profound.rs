extern crate hyper;
extern crate slack;
extern crate rand;

use hyper::Client;
use listener::{MessageListener, Message};
use regex::Regex;
use rustc_serialize::json;
use self::rand::Rng;
use std::io::Read;


/// Listens for the words `be profound` and posts a random profound programmer link
///
/// Example @yobot be profound
pub struct ProfoundListener {
    regex: Regex,
}

#[derive(RustcDecodable)]
struct ProfoundPost {
    photo_url_1280: String,
}

#[derive(RustcDecodable)]
struct ProfoundResponse {
    posts: Vec<ProfoundPost>,
}

impl ProfoundResponse {
    fn new() -> ProfoundResponse {
        ProfoundResponse {
            posts: Vec::new(),
        }
    }
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
        let mut rng = rand::thread_rng();
        let index = rng.next_u32() % 116;
        let url = format!("http://theprofoundprogrammer.com/api/read/json?debug=1&num=1&type=photo&start={}", index);
        if let Ok(mut response) = client.get(&url).send() {
            let mut body = String::new();
            let _ = response.read_to_string(&mut body);
            let parts: Vec<&str> = body.split("-").collect();
            let string = parts.join("_");
            let response = json::decode(&string).unwrap_or(ProfoundResponse::new());
            if let Some(post) = response.posts.first() {
                let _ = cli.send_message(&message.channel, &post.photo_url_1280);
                return;
            }
        }

        let _ = cli.send_message(&message.channel, "Maybe next time");
    }
}
