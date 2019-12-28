extern crate peerchat;
extern crate serde_json;
#[macro_use] extern crate log;

use peerchat::protocol::*;
use peerchat::utils::setup_log;

use std::thread;
use std::net::UdpSocket;
use std::{io, env};
use std::str::from_utf8;

static SEND_IP: &str      = "127.0.0.1:54322";
static BROADCAST_IP: &str = "127.0.0.1:54321";

fn post_loop(name: &str) {
    send_arrive(name);
    loop {
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if let Some(input) = input.lines().nth(0){
                send_as(name, input)
            }
        } else {
            break;
        }
    }
}

fn send_as(name: &str, msg:&str) {
    send_msg(
        &Message::Message {
            name: String::from(name),
            payload: MessagePayload::Message{content:msg.into()},
            address: SEND_IP.parse().unwrap()
        }
    );
}

fn send_arrive(name: &str) {
    send_msg(&Message::Message{name: String::from(name), payload: MessagePayload::Arrived,
            address: SEND_IP.parse().unwrap()
    });
}

fn send_announce(name: &str) {
    send_msg(&Message::Message{name: String::from(name), payload: MessagePayload::Announce,
            address: SEND_IP.parse().unwrap()
    });
}

fn send_msg(msg: &Message) {
    let msg = serde_json::to_string(msg).unwrap();

    let socket = UdpSocket::bind(SEND_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    socket.send_to(&msg.as_bytes(), BROADCAST_IP).unwrap();
}

/// Sends a bad message on purpose
fn send_bad() {
    let msg = "Can't parse this!! Da Da Da! Da Da! Da Da!";

    let socket = UdpSocket::bind(SEND_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    socket.send_to(&msg.as_bytes(), BROADCAST_IP).unwrap();
}

fn receive(my_name: &str) {
    info!("listening ...");
    let socket = UdpSocket::bind(BROADCAST_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    let mut buf = [0; 4096];
    while let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        let msg = Message::from_bytes(&buf[0..amt]);

        match msg {
            Message::Message{name, payload, ..} => {
                match payload  {
                    MessagePayload::Message{content} => println!("{}: {:?}", name, content),
                    MessagePayload::Arrived => {
                        println!("---- \"{}\" entered the room", name);
                        send_announce(my_name);
                    },
                    MessagePayload::Announce => println!("---- {} ", name),
                    MessagePayload::WhosThere => send_announce(my_name),
                    msg @ _ => warn!("unhandled message: {:?}", msg)
                }
            },
            Message::Error => error!("---- Error! can't deserialize {}", from_utf8(&buf[0..amt]).unwrap()),
        }
    }
}

fn main() {
    setup_log();
    trace!("starting up");
    trace!("args {:?}", env::args().collect::<Vec<_>>());
    if let Some(arg) = env::args().nth(1) {
        trace!("arg[1] == {}", arg );
        if env::args().nth(1).is_some() {
            let name = env::args().skip(2).collect::<Vec<_>>().join("_");
            if arg == "send" { post_loop(&name); }
            if arg == "send_bad" { send_bad(); }
            if arg == "recv" { receive(&name); }
            if arg == "both" {
                {
                    let name = name.clone();
                thread::spawn(move || receive(&name));
                }
                send_arrive(&name);
                post_loop(&name);
            }
        } else { println!("Please enter a Name") }
    } else { println!("What? please use \"send\" or \"recv\" or \"both\"") }
}

