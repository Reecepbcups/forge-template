use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::reqwest,
};
use bindings::wavsservicemanager::WavsServiceManager;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = reqwest::Url::parse("http://127.0.0.1:8545")?;
    let provider: RootProvider<_> = ProviderBuilder::new().on_http(url).boxed();

    let address = "0x0000000000000000000000000000000000000000".parse::<Address>()?;

    let _contract = WavsServiceManager::new(address, provider.clone());

    let blk = provider.get_block_number().await?;
    println!("Hello, world! {}", blk);
    Ok(())
}
