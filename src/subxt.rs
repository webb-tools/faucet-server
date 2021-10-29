use serde::{Deserialize, Serialize};
use sp_keyring::AccountKeyring;
use substrate_subxt::{balances::*, ClientBuilder, KusamaRuntime, PairSigner};

#[derive(Deserialize, Serialize, Clone)]
pub struct Transfer {
    account: String,
    amount: i64,
}

pub async fn subxtTransfer() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // let pair_from = Pair::from_string_with_seed(&transfer_info.account, Option::None);
    // let signer = PairSigner::new(pair_from);

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let client = ClientBuilder::<KusamaRuntime>::new().build().await?;
    let hash = client.transfer(&signer, &dest, 10_000).await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
