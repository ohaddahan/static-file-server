use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long)]
    pub dir: String,
    #[arg(long, default_value = "3000")]
    pub port: u16,
    #[arg(long, default_value = "true")]
    pub cors: bool,
    #[arg(long, default_value = "true")]
    pub compression: bool,
}
