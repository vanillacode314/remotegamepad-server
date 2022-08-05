use clap::Parser;
use enigo::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error};
use ws::{Handler, Message, Sender, WebSocket};

/// Remote gamepad server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host to bind server on
    #[clap(short, long, value_parser, default_value = "0.0.0.0")]
    host: String,

    /// Port
    #[clap(short, long, value_parser, default_value_t = 8080)]
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct CmdMessage {
    command: Cmd,
}

#[derive(Serialize, Deserialize, Debug)]
enum Cmd {
    Hold(String),
    Press(String),
    Release(String),
}

struct MsgHandler {
    sender: Sender,
}

impl Handler for MsgHandler {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        println!("Connection Successful with client");
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let sender = &self.sender;
        if let Message::Text(str) = msg {
            if let Ok::<CmdMessage, Error>(message) = serde_json::from_str(str.as_str()) {
                perform_key_command(&message);
                let value = json!({
                    "result": "success"
                });
                sender.send(serde_json::to_string(&value).expect("convert return value to String"))
            } else {
                sender.send("not a valid command object")
            }
        } else {
            sender.send("not a valid command object")
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ws = WebSocket::new(|sender| MsgHandler { sender }).expect("create websocket");
    if let Ok(ws) = ws.bind(format!("{}:{}", args.host, args.port)) {
        println!("Server started successfully on {}:{}", args.host, args.port);
        ws.run().expect("start websocket server");
    } else {
        println!("Server start failed");
    }
}

fn perform_key_command(msg: &CmdMessage) {
    let mut enigo = Enigo::new();
    match &msg.command {
        Cmd::Hold(key_str) => {
            if let Some(key) = get_key(key_str) {
                enigo.key_down(key);
            } else {
                let chars: Vec<char> = key_str.chars().collect();
                enigo.key_down(Key::Layout(chars[0]));
            }
        }
        Cmd::Press(key_str) => {
            if let Some(key) = get_key(key_str) {
                enigo.key_down(key);
                enigo.key_up(key);
            } else {
                let chars: Vec<char> = key_str.chars().collect();
                enigo.key_down(Key::Layout(chars[0]));
                enigo.key_up(Key::Layout(chars[0]));
            }
        }
        Cmd::Release(key_str) => {
            if let Some(key) = get_key(key_str) {
                enigo.key_up(key);
            } else {
                let chars: Vec<char> = key_str.chars().collect();
                if chars.len() > 0 {
                    enigo.key_up(Key::Layout(chars[0]));
                }
            }
        }
    }
}
fn get_key(key_str: &str) -> Option<Key> {
    match key_str {
        "up" => Some(Key::UpArrow),
        "down" => Some(Key::DownArrow),
        "left" => Some(Key::LeftArrow),
        "right" => Some(Key::RightArrow),
        _ => None,
    }
}
