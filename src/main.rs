use clap::{arg, command, Parser};
use warp::Filter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(version = "1.0", author = "Jelf")]
struct Opts {
    #[arg(short, long)]
    dir: Option<String>,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    println!("opts: {:?}", opts);

    let work_dir = match opts.dir {
        Some(s) => s,
        None => std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    dbg!(&work_dir);

    let route = warp::get().and(warp::fs::dir(work_dir));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
