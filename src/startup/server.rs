use crate::cli::CliArgs;
use crate::terminal::colored_msg;
use anyhow::anyhow;
use axum::http::{header, HeaderValue};
use axum::Router;
use std::fs;
use std::net::SocketAddr;
use termcolor::Color;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::set_header::{SetRequestHeaderLayer, SetResponseHeaderLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct Server {
    port: u16,
    listener: TcpListener,
    addr: String,
    cli_args: CliArgs,
    routes: Router,
}

impl Server {
    pub async fn new(cli_args: &CliArgs) -> anyhow::Result<Server> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "static_file_server=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        let routes = Server::routes(&cli_args)?;
        let listener = Server::listen(&cli_args).await?;
        let port = listener.local_addr()?.port();

        Ok(Server {
            port,
            addr: format!("http://localhost:{}", port),
            cli_args: cli_args.clone(),
            listener,
            routes,
        })
    }

    fn routes(cli_args: &CliArgs) -> anyhow::Result<Router> {
        if fs::metadata(&cli_args.dir).is_err() {
            colored_msg(&format!("Dir {} does not exist", cli_args.dir), Color::Red)?;
            return Err(anyhow!(format!("Dir {} does not exist", cli_args.dir)));
        } else {
            colored_msg(&format!("Found dir {}", cli_args.dir), Color::Green)?;
        }
        let mut routes: Router;
        routes = if cli_args.compression_dir {
            Router::new().nest_service("/", ServeDir::new(&cli_args.dir).precompressed_gzip())
        } else {
            Router::new().nest_service("/", ServeDir::new(&cli_args.dir))
        };
        routes = if cli_args.compression_response {
            routes
                .layer(tower_http::compression::CompressionLayer::new())
                .layer(SetRequestHeaderLayer::if_not_present(
                    header::ACCEPT_ENCODING,
                    HeaderValue::from_static("gzip"),
                ))
        } else {
            routes
        };
        routes = if cli_args.cors {
            routes
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::ACCESS_CONTROL_ALLOW_METHODS,
                    HeaderValue::from_static("*"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    HeaderValue::from_static("*"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_static("*"),
                ))
        } else {
            routes
        };
        Ok(routes)
    }

    async fn listen(args: &CliArgs) -> anyhow::Result<TcpListener> {
        let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
        let listener = TcpListener::bind(addr).await?;
        Ok(listener)
    }

    pub async fn serve(self) -> anyhow::Result<()> {
        colored_msg(&format!("Options:\n{:#?}", &self.cli_args,), Color::Green)?;
        colored_msg(
            &format!(
                "Listening on = http://localhost:{} | {}",
                &self.port, &self.addr
            ),
            Color::Green,
        )?;
        Ok(
            axum::serve(self.listener, self.routes.layer(TraceLayer::new_for_http()))
                .await
                .unwrap(),
        )
    }
}
