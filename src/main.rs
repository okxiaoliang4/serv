#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("."))
        .run(([127, 0, 0, 1], 3030))
        .await;
}