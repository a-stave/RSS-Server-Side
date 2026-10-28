use std::net::SocketAddr;

use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// The socket address on which to listen for incoming client connections.
    #[arg(default_value = "[::1]:8080")]
    listen: SocketAddr,
}

fn main() {
    let args = Args::parse();

    println!("Starting RSS-Server-Side");
    println!("listen = {}", args.listen);
}
