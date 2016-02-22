extern crate log;
extern crate regex;
extern crate yobot;

use yobot::Yobot;
use log::{LogLevelFilter};
use yobot::listener::*;

mod logger;

fn main() {
    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(logger::SimpleLogger)
    });

    Yobot::new()
        .add_listener(meme::MemeListener::new())
        .add_listener(echo::EchoListener::new())
        .connect();
}
