use std::net::{SocketAddr, TcpStream};

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
    println!("Will listen on this socket: {}", args.listen);

    let run_result = run_server(&args.listen);
    match run_result {
        Ok(()) => {
            println!("Server exited successfully.")
        }
        Err(e) => {
            eprintln!("Server exited with error: {e}");
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ServerRunError {
    #[error("Failed to bind a TCP listener: {0}")]
    ListenerBindFailed(std::io::Error),
}

fn run_server(listen_addrs: &SocketAddr) -> Result<(), ServerRunError> {
    let listener =
        std::net::TcpListener::bind(listen_addrs).map_err(ServerRunError::ListenerBindFailed)?;

    println!("Entering TCP listener loop...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle_result = handle_client(stream);
                match handle_result {
                    Ok(()) => {
                        println!("Client session concluded successfully.");
                    }
                    Err(client_err) => {
                        eprintln!("Client session concluded with error: {client_err}");
                    }
                }
            }
            Err(conn_accept_err) => {
                eprintln!("Bad new connection: {conn_accept_err}");
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum HandleClientError {
    #[error("Failed to get the client's socket address: {0}")]
    PeerAddrFailed(std::io::Error),
}

fn handle_client(stream: TcpStream) -> Result<(), HandleClientError> {
    let peer_addr = stream
        .peer_addr()
        .map_err(HandleClientError::PeerAddrFailed)?;
    println!("New connection from client at {peer_addr}");
    Ok(())
}
