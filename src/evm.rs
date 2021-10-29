use anyhow::Result;
use ethers::{prelude::*, utils::Ganache};
use std::convert::TryFrom;

pub async fn evmTransfer() -> Result<()> {
    let ganache = Ganache::new().spawn();

    // connect to the network
    let provider = Provider::<Http>::try_from(ganache.endpoint())?;
    let accounts = provider.get_accounts().await?;
    let from = accounts[0];
    let to = accounts[1];

    // craft the tx
    let tx = TransactionRequest::new().to(to).value(1000).from(from); // specify the `from` field so that the client knows which account to use

    let balance_before = provider.get_balance(from, None).await?;

    // broadcast it via the eth_sendTransaction API
    let tx = provider.send_transaction(tx, None).await?.await?;

    println!("{}", serde_json::to_string(&tx)?);

    let balance_after = provider.get_balance(from, None).await?;

    println!("Balance before {}", balance_before);
    println!("Balance after {}", balance_after);

    Ok(())
}