extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::thread;
use std::net::UdpSocket;
use std::{io, env};
use std::str::from_utf8;


static SEND_IP:&str      = "255.255.255.255:54322";
static BROADCAST_IP:&str = "255.255.255.255:54321";

#[derive(Serialize, Deserialize)]
enum Message {
    Message {
        name: String,
        payload: MessagePayload,
        //sent_timestamp
    },
    Error
}

#[derive(Serialize, Deserialize)]
enum MessagePayload{
    Message {
        content: String,
    },
    Arrived,
    Announce,
    WhosThere
}

fn post_loop(name:&str) {
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
    send_msg(&Message::Message{name: name.into(), payload: MessagePayload::Message{content:msg.into()}});
}

fn send_arrive(name: &str) {
    send_msg(&Message::Message{name: name.into(), payload: MessagePayload::Arrived});
}

fn send_announce(name: &str) {
    send_msg(&Message::Message{name: name.into(), payload: MessagePayload::Announce});
}

fn send_msg(msg: &Message) {
    let socket = UdpSocket::bind(SEND_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    let msg = serde_json::to_string(msg).unwrap();
    socket.send_to(&msg.as_bytes(), BROADCAST_IP).unwrap();
}

fn receive(my_name:&str) {
    let socket = UdpSocket::bind(BROADCAST_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    let mut buf = [0; 4096];
    while let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        let msg = serde_json::from_slice::<Message>(&buf[0..amt])
            .unwrap_or(Message::Error);

        match msg {
            Message::Message{name, payload} => {
                match payload  {
                    MessagePayload::Message{content} => println!("{}: {:?}", name, content),
                    MessagePayload::Arrived => {
                        println!("---- \"{}\" entered the room", name);
                        send_announce(my_name);
                    },
                    MessagePayload::Announce => println!("---- {} ", name),
                    MessagePayload::WhosThere => send_announce(my_name),
                }
            },
            Message::Error => println!("---- Error! can't deserialize {}", from_utf8(&buf[0..amt]).unwrap()),
        }
    }
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        if env::args().nth(2).is_some() {
            let name = env::args().skip(2).collect::<Vec<_>>().join("_");
            if arg == "send" {
                post_loop(&name);
            }
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

