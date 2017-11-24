#[macro_use] extern crate log;
extern crate peerchat;

use std::env;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::str::from_utf8;

use peerchat::protocol::*;
use peerchat::utils::setup_log;


fn start_server(addr: &SocketAddr) {
    let (sender, receiver) = channel();

    let addr = addr.to_owned();
    thread::spawn(move || listening_thread(addr, sender));
    thread::spawn(move || broadcasting_thread(receiver));
}

fn listening_thread(addr: SocketAddr, sender: Sender<Message>) {
    info!("listening on {} ...", addr);
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    let mut buf = [0; 4096];
    while let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        let msg = decode_msg(&buf[0..amt]);
        if let Message::Error = msg {
            error!("---- Error! can't deserialize {}", from_utf8(&buf[0..amt]).unwrap());
        }
        debug!("received {:#?}", msg);
        sender.send(msg).unwrap();
    }
}

fn broadcasting_thread(receiver: Receiver<Message>) {

    //let mut participants = Vec::new();

    loop {
        let msg = receiver.recv().unwrap();
        match msg {
            Message::Message{name, payload, ..} => {
                match payload  {
                    MessagePayload::Message{content} => println!("{}: {:?}", name, content),
                    msg @ _ => warn!("unhandled message: {:?}", msg)
                }
            }
            _ => {trace!("not broadcasting, error occured earlier")}
        }
    }
}

fn broadcast(msg: &str, receivers: &[SocketAddr]) {

}

fn main() {
    setup_log();
    trace!("starting up");
    trace!("args {:?}", env::args().collect::<Vec<_>>());

    if let Some(addr_str) = env::args().nth(1) {
        let addr: SocketAddr = addr_str.parse().unwrap();
        start_server(&addr);
    } else {
        println!("Please enter the IP to listen on")
    }

    ::std::io::stdin().read_line(&mut String::new()).unwrap();


}