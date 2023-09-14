use axum::{
    body::Body,
    handler::HandlerWithoutStateExt,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use clap::{arg, command, Parser};
use std::{io, net::SocketAddr};
use tower_http::services::{ServeFile, ServeDir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(version = "1.0", author = "Jelf")]
struct Opts {
    #[arg(short, long)]
    dir: Option<String>,
}

#[tokio::main]
async fn main() {
    let current_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let opts = Opts::parse();
    let workdir = opts.dir.or(Some(current_dir)).unwrap();

    let compression = String::from("br");

    dbg!(&workdir);

    let app = Router::new()
        .nest_service("/", get_service(ServeDir::new(workdir)).handle_error(|error: io::Error| async move {
          (
              StatusCode::INTERNAL_SERVER_ERROR,
              format!("Unhandled internal error: {}", error),
          )
      }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
