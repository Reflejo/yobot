# yobot
Slack bot that can be extended with listners.

Yobot is an extensible slack bot. You can add listeners that define a regex in order to
handle real time events on a slack channel.

Settting `SLACK_BOT_TOKEN` env variable and running `cargo run` will get you a bot,
just invite the bot to the channels you want on slack and start the fun.

## Listeners

Listeners provide a `Regex` which the main loop uses to check whether the listener is interested.
If the regex matches, `handle` is called on the handler with the Message and RtmClient. The
listener can then do some work and use the client to send its response.

Note that a panic on a listener (or a crash) will crash the entire client.

Implementing a `MessageListener` enables responding to `slack::Message`s. There
are currently very few requirements to creating a handler. The
[`handle`](#method.handle) function receives a `slack::Message` and a `slack::RtmClient`.
The listener is responsible for testing whether it's interested in replying by
defining a regular expression on [`re`](#method.re)
Optionally call `cli.send_message` to send a response.

### Example

A simple echo handler might look something like the following:

```rust
use regex::Regex;
use yobot::listener::{MessageListener, Message};

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
        "echo"
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let _ = cli.send_message(&message.channel, &message.text);
    }
}
```

## Yobot

Yobot is the main struct of the bot. Add a bunch of listeners and you call `connect` to connect
the real time API and start listening for messages.

### Example

```rust
use yobot::Yobot;

let yobot = Yobot::new();
yobot.add_listener(??);
yobot.connect();
```
