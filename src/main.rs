use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(version = "1.0", author = "Jelf")]
struct Opts {
    #[arg(short, long)]
    dir: String,
}

#[tokio::main]
async fn main() {
    let currentDir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // TODO: default to current dir
    let opts = Opts::parse();
    println!("opts: {:?}", opts);

    warp::serve(warp::fs::dir(opts.dir))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
