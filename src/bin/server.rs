use dnet::init;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    init().await?;
    Ok(())
}
