use zen_steps::error::Error;
use zen_steps::server::run_server;
use zen_steps::shared::setup;

#[tokio::main]
async fn main()-> Result<(), Error> {
    setup()?;
    run_server().await?;
    Ok(())
}