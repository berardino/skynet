extern crate colored;
extern crate pnet;

use colored::*;
use pnet::datalink::{self};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;


fn main() {
    let interfaces = datalink::interfaces();

    for interface in &interfaces {
        println!("{}", "---------------------------".blue());
        println!("{}", interface.to_string().green());
    }

    let interface_name = "en0";

    let en0 = interfaces.iter()
        .filter(|iface| iface.name == interface_name)
        .next()
        .unwrap();

    println!();
    println!("{} {}", "**** Found interface".yellow().bold(), interface_name);
    println!();

    // int socket(int domain, int type, int protocol);
    // http://man7.org/linux/man-pages/man2/socket.2.html
    let (_, mut rx) = match datalink::channel(&en0, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(p) => {
                let ethernet = EthernetPacket::new(p).unwrap();

                match ethernet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        let header = Ipv4Packet::new(ethernet.payload()).unwrap();
                        println!("{}", format!("{:?}", header).magenta());
                        match header.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                let tcp = TcpPacket::new(header.payload()).unwrap();
                                println!("{}", format!("{:?}", tcp).green());
                            }
                            _ => {}
                        }
                    }
                    EtherTypes::Ipv6 => {
                        let header = Ipv6Packet::new(ethernet.payload()).unwrap();
                        println!("{}", format!("{:?}", header).yellow());
                        match header.get_next_header() {
                            IpNextHeaderProtocols::Tcp => {
                                let tcp = TcpPacket::new(header.payload()).unwrap();
                                println!("{}", format!("{:?}", tcp).green());
                            }
                            _ => {}
                        }
                    }
                    EtherTypes::Arp => {
                        println!("Arp");
                    }
                    _ => {
                        println!("Not supported");
                    }
                }
                println!("{}", format!("{:?}", ethernet).blue());
                //println!("{:?}", ethernet.payload());
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
