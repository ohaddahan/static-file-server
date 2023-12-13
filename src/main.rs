use axum::Router;
use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::process::exit;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dir: String,
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

fn colored_msg(msg: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    tracing::debug!("{}", msg);
    writeln!(&mut stdout, "{}", msg)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    if fs::metadata(&args.dir).is_err() {
        colored_msg(&format!("Dir {} does not exist", args.dir), Color::Red).unwrap_or_default();
        exit(1);
    } else {
        colored_msg(&format!("Found dir {}", args.dir), Color::Green).unwrap_or_default();
    }

    colored_msg(
        &format!("Serving dir {} at http://localhost:3000", args.dir),
        Color::Green,
    )
    .unwrap_or_default();
    tokio::join!(serve(
        Router::new().nest_service("/", ServeDir::new(args.dir)),
        args.port
    ),);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    colored_msg(
        &format!("Listening on {}", listener.local_addr().unwrap()),
        Color::White,
    )
    .unwrap_or_default();
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
