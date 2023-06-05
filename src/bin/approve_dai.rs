use std::sync::Arc;

use ethers::{
    abi::Address,
    prelude::{abigen, k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::Wallet,
    types::U256,
};

const RPC_URL: &str = "https://sphinx.shardeum.org";
const DAI_ADDRESS: &str = "0x559787297A9E3E558C764a647c7877d84DAE1c05";

/// Router address
const SPENDER_ADDRESS: &str = "0xeed4ef0Fc40c3B13a9f85eDcf1879609281BCA94";
const APPROVAL_AMOUNT: &str = "10000000000000";

#[tokio::main]
async fn main() {
    abigen!(Dai, "abis/Dai.json");

    let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
    let wallet = std::env::var("PRIVATE_KEY")
        .unwrap()
        .parse::<Wallet<SigningKey>>()
        .unwrap();
    let signer_middleware = SignerMiddleware::new(provider, wallet);

    let dai_address = DAI_ADDRESS.parse::<Address>().unwrap();
    let dai = Dai::new(dai_address, Arc::new(signer_middleware));

    let spender_address = SPENDER_ADDRESS.parse::<Address>().unwrap();
    let approval_amount = APPROVAL_AMOUNT.parse::<U256>().unwrap();

    let call = dai.approve(spender_address, approval_amount).legacy();

    // broadcast the transaction, log its hash, and wait for it to be confirmed
    call.send()
        .await
        .unwrap()
        .log_msg("approving! here's the tx hash")
        .await
        .unwrap();

    println!("done!");
}
