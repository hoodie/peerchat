#[macro_use] extern crate log;
extern crate peerchat;
extern crate serde_json;

use std::env;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::str::from_utf8;

use peerchat::protocol::*;
use peerchat::utils::setup_log;

static SEND_IP: &str      = "127.0.0.1:54322";
static BROADCAST_IP: &str = "127.0.0.1:54321";

fn start_server(addr: &SocketAddr) {
    let (tx_recv_message, rx_recv_message) = channel();
    let (tx_send_message, rx_send_message) = channel();


    let addr = addr.clone();
    let send_addr = SocketAddr::new(
        addr.ip(),
        addr.port()+1,
        );

    thread::spawn(move || listening_thread(addr, tx_recv_message));
    thread::spawn(move || handling_thread(rx_recv_message, tx_send_message));
    thread::spawn(move || sending_thread(send_addr, rx_send_message));
}

fn listening_thread(addr: SocketAddr, sender: Sender<Message>) {
    info!("listening on {} ...", addr);
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    let mut buf = [0; 4096];

    while let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        let msg = Message::from_bytes(&buf[0..amt]);
        if let Message::Error = msg {
            error!("---- Error! can't deserialize {}", from_utf8(&buf[0..amt]).unwrap());
        }
        debug!("received {:#?}", msg);
        sender.send(msg).unwrap();
    }
}

fn sending_thread(addr: SocketAddr, receiver: Receiver<Message>) {
    info!("sending via {} ...", addr);

    let socket = UdpSocket::bind(SEND_IP).expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");
    loop {
        let msg = receiver.recv().unwrap();
        let msg = serde_json::to_string(&msg).unwrap();
        socket.send_to(&msg.as_bytes(), BROADCAST_IP).unwrap();
    }
}

fn handling_thread(receiver: Receiver<Message>, sender: Sender<Message>) {
    let mut participants = Vec::new();

    loop {
        let msg = receiver.recv().unwrap();
        match msg {
            Message::Message{ref name, ref payload, address} => {
                match payload  {
                    &MessagePayload::Message{..} => sender.send(msg.clone()).unwrap(),
                    &MessagePayload::Arrived => participants.push(address),
                    msg @ _ => warn!("unhandled message: {:?}", msg)
                }
            }
            _ => {trace!("not broadcasting, error occured earlier")}
        }
        trace!("participants list {:#?}", participants);
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
