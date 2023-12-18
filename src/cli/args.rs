use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Directory to serve
    #[arg(long, required = true, default_value = ".")]
    pub dir: String,
    /// Port to listen on, use 0 to select a random port
    #[arg(long, default_value = "3000", required = false)]
    pub port: u16,
    /// Enable CORS [default: true]
    #[arg(long, action)]
    pub cors: bool,
    /// Enable dir compression [default: true]
    #[arg(long, action)]
    pub compression_dir: bool,
    /// Enable response compression [default: true]
    #[arg(long, action)]
    pub compression_response: bool,
}
