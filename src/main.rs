use clap::Parser;
use macaddr::MacAddr6;
use std::net::UdpSocket;

#[derive(Parser, Debug)]
#[command(name = "wol-cli")]
#[command(version, about)]
struct Args {
    #[arg(short ='m', long)]
    mac: MacAddr6,
    #[arg(short = 'b' , long, default_value_t = String::from("255.255.255.255"))]
    ipaddr: String,
    #[arg(short = 'p', long, default_value_t = 9)]
    port: u16,
}

fn build_magic_packet(mac: [u8; 6]) -> [u8; 102] {
    todo!()
}

fn main() {
    let args = Args::parse();
    let mac: [u8;6] = args.mac.into_array();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();

    let broadcast: String  = format!("{}:{}", args.ipaddr, args.port);
    socket.send_to(&build_magic_packet(mac), broadcast).unwrap();
}