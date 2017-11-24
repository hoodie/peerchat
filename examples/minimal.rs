extern crate peerchat;
extern crate serde_json;
#[macro_use] extern crate log;

use peerchat::protocol::*;
use peerchat::utils::setup_log;

use std::net::UdpSocket;
use std::{io, env};
use std::str::from_utf8;

static SEND_IP: &str      = "10.1.135.148:54322";
static BROADCAST_IP: &str = "10.1.135.148:54321";

fn post_loop(name:&str) {
    loop{
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if let Some(msg) = input.lines().nth(0){

    send_msg(
        &Message::Message {
            name: String::from(name),
            payload: MessagePayload::Message{content: msg.into()},
            address: SEND_IP.parse().unwrap()
        }
    );
            }
        } else {
            break;
        }
    }
}


fn send_msg(msg: &Message) {
    let msg = serde_json::to_string(msg).unwrap();

    let socket = UdpSocket::bind(SEND_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    socket.send_to(&msg.as_bytes(), BROADCAST_IP).unwrap();
}

fn receive() {
    info!("listening ...");
    let socket = UdpSocket::bind(BROADCAST_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");

    let mut buf = [0; 4096];
    while let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        let msg = decode_msg(&buf[0..amt]);

        match msg {
            Message::Message{name, payload, ..} => {
                match payload  {
                    MessagePayload::Message{content} => println!("{}: {:?}", name, content),
                    msg @ _ => warn!("unhandled message: {:?}", msg)
                }
            },
            Message::Error => error!("---- Error! can't deserialize {}", from_utf8(&buf[0..amt]).unwrap()),
        }
    }
}

fn main() {
    setup_log();
    if let Some(arg) = env::args().nth(1) {
        trace!("arg[1] == {}", arg );
        if env::args().nth(1).is_some() {
            let name = env::args().skip(2).collect::<Vec<_>>().join("_");
            if arg == "send" { post_loop(&name); }
            if arg == "recv" { receive(); }
        } else { println!("Please enter a Name") }
    } else { println!("What? please use \"send\" or \"recv\" or \"both\"") }
}

