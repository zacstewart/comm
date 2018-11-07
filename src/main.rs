extern crate env_logger;
extern crate crypto;
#[macro_use]
extern crate log;
extern crate mio;
extern crate num;
extern crate protobuf;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashSet;
use std::env;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::mpsc;
use std::thread;

use address::Address;
use client::Task;
use client::messages::TextMessage;

mod address;
mod client;
mod messages;
mod network;
mod node;
mod node_bucket;
mod routing_table;
mod servers;
mod stun;
mod transaction;

/// Starts a command line client.
///
/// Usage:
///
///     comm SECRET LOCAL_ADDR BOOTSTRAP_ADDR
///
/// Example:
///
///     comm alpha 0.0.0.0:6667 10.0.1.13
fn main() {
    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let secret = args[1].clone();

    let address = Address::for_content(secret.as_str());

    let host = args[2].as_str().to_socket_addrs().unwrap().next().unwrap();
    let servers = vec![servers::Server::Udp(servers::UdpServer::new(host))];

    let routers: Vec<node::Node> = match args.get(3) {
        Some(router_address) => {
            let mut transports = HashSet::new();
            transports.insert(node::Transport::Udp(node::UdpTransport::new(
                        router_address.as_str().to_socket_addrs().unwrap().next().unwrap())));
            let router_node = node::Node::new(Address::null(), transports);
            vec![router_node]
        }
        None => vec![]
    };

    let network = network::Network::new(address, servers, routers);
    let mut client = client::Client::new(address);
    let (event_sender, events) = mpsc::channel();
    client.register_event_listener(event_sender);
    let client_channel = client.run(network);

    thread::spawn(move || {
        for event in events {
            println!("Event: {:?}", event);
        }
    });

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let recipient = Address::from_str(parts[0]).unwrap();
            let message_text = parts[1].trim().to_string();

            let text_message = TextMessage::new(address, message_text);
            client_channel
                .send(Task::ScheduleMessageDelivery(recipient, text_message))
                .unwrap_or_else(|err| info!("Couldn't schedule message delivery: {:?}", err));
        }
    }
}
