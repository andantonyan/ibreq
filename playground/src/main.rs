#![allow(warnings)]

use pnet::{
  datalink::{self, Channel::Ethernet, NetworkInterface},
  ipnetwork::IpNetwork,
  packet::{
    ip::IpNextHeaderProtocols,
    tcp::{MutableTcpPacket, TcpFlags, TcpOption},
  },
  transport::{transport_channel, TransportChannelType::Layer4, TransportProtocol},
};
use std::net::{IpAddr, Ipv4Addr};

fn main() {
  let interfaces = datalink::interfaces();
  let interface_default_match = |e: &&NetworkInterface| {
    e.is_up()
      && !e.is_loopback()
      && e.ips.len() > 0
      && e.ips.iter().any(|ip| match ip {
        IpNetwork::V4(_) => true,
        IpNetwork::V6(_) => false,
      })
  };

  let interface = interfaces
    .iter()
    .filter(interface_default_match)
    .next()
    .unwrap();

  println!("Default interface\n\n{:?}\n", interface);

  println!("Creating layer 2 channel...");
  let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
    Ok(Ethernet(tx, rx)) => (tx, rx),
    Ok(_) => panic!("Unhandled channel type"),
    Err(e) => panic!(
      "An error occurred when creating the datalink channel: {}",
      e
    ),
  };
  println!("Done creating layer 2 channel.");

  println!("Creating layer 4 channel...");
  let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp));
  let (mut tx, mut rx) = match transport_channel(4096, protocol) {
    Ok((tx, rx)) => (tx, rx),
    Err(e) => panic!(
      "An error occurred when creating the transport channel: {}",
      e
    ),
  };
  println!("Done creating layer 4 channel.");
}
