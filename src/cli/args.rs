use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Directory to serve
    #[arg(long, default_value = ".")]
    pub dir: String,
    /// Port to listen on, use 0 to select a random port
    #[arg(long, default_value = "3000")]
    pub port: u16,
    /// Enable CORS [default: false]
    #[arg(long, default_value = "false")]
    pub cors: bool,
    /// Enable dir compression [default: false]
    #[arg(long, default_value = "false")]
    pub compression_dir: bool,
    /// Enable response compression [default: false]
    #[arg(long, default_value = "false")]
    pub compression_response: bool,
}
