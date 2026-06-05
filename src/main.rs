use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wol-cli")]
#[command(version, about)]
struct Args {
    #[arg(short ='m', long)]
    mac: String,
    #[arg(short = 'b' , long, default_value_t = String::from("255.255.255.255"))]
    broadcast: String,
    #[arg(short = 'p', long, default_value_t = 9)]
    port: u16,
}

fn main() {
    todo!()
}