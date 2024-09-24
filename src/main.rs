use server::create_server;

mod server;
mod technical_endpoints;

use colored::Colorize;

#[tokio::main]
async fn main() {
    match start().await {
        Ok(_) => println!("finished run with no errors"),
        Err(err) => println!("{}", err.to_string().red()),
    }
}

pub async fn start() -> anyhow::Result<()> {
    let (listener, app) = create_server().await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
