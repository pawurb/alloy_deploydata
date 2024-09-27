use alloy::{node_bindings::Anvil, providers::ProviderBuilder, sol};
use anyhow::Result;

sol!(
    #[sol(rpc)]
    WETH,
    "./deploydata/WETH.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // https://etherscan.io/address/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2#code
    let anvil = Anvil::new().try_spawn()?;
    let provider = ProviderBuilder::new().on_http(anvil.endpoint().parse()?);

    let weth = WETH::deploy(provider.clone()).await?;

    let name = weth.name().call().await?._0;

    println!("Name: {}", name);
    Ok(())
}
