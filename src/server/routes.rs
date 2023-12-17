use crate::cli::CliArgs;
use axum::http::{header, HeaderValue};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::set_header::{SetRequestHeaderLayer, SetResponseHeaderLayer};

pub fn create_routes(cli_args: &CliArgs) -> Router {
    let mut routes = if cli_args.compression {
        Router::new()
            .nest_service("/", ServeDir::new(&cli_args.dir).precompressed_gzip())
            .layer(tower_http::compression::CompressionLayer::new())
            .layer(SetRequestHeaderLayer::if_not_present(
                header::ACCEPT_ENCODING,
                HeaderValue::from_static("gzip"),
            ))
    } else {
        Router::new().nest_service("/", ServeDir::new(&cli_args.dir))
    };

    if cli_args.cors {
        routes = routes
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
            ));
    }
    routes
}
