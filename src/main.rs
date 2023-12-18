mod cli;
mod startup;
mod terminal;

use crate::cli::CliArgs;
use crate::startup::Server;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let server = Server::new(&args).await.unwrap();
    let server_task = tokio::spawn(server.serve());
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl-c received, shutting down");
        }
        _ = server_task => {
            println!("Server task finished");
        }
    }
    // tokio::join!(server_task,);
}
