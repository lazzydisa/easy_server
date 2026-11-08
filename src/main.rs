
use axum::{
    routing::get,
    Router,
    response::Html,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -r -- \"ip:port\"");
        std::process::exit(1);
    }

    let listener = tokio::net::TcpListener::bind(&args[1]).await?;
	let app = Router::new().route("/", get(async || { Html(tokio::fs::read_to_string("sources/root.html").await.unwrap()) }))
		.route("/root.html", get(async || { Html(tokio::fs::read_to_string("sources/root.html").await.unwrap()) }))
		.route("/channels.html", get(async || { Html(tokio::fs::read_to_string("sources/channels.html").await.unwrap()) }))
		.route("/socials.html", get(async || { Html(tokio::fs::read_to_string("sources/socials.html").await.unwrap()) }));

    println!("{:?}", listener);
    axum::serve(listener, app).await?;
    Ok(())
}
