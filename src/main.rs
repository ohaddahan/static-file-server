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
    tokio::join!(server_task,);
}
