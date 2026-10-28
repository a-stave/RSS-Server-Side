use std::net::SocketAddr;

use clap::Parser;

#[derive(clap::Parser)]
/// TODO: a short summary message can go here.
///
/// TODO: a more detailed description of the command can go here,
/// to be shown when the long `--help` flag is used, but not for
/// the short `-h`.
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
