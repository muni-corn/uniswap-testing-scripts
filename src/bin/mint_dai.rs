use std::sync::Arc;

use ethers::{abi::Address, prelude::abigen, types::U256};
use uniswap_testing::get_signer_middleware;

const DAI_ADDRESS: &str = "0x559787297A9E3E558C764a647c7877d84DAE1c05";

const RECEIPIENT_ADDRESS: &str = "0xAE2c4A62737E02973Cc2955d5e98264EcDc4Aa12";
const MINT_AMOUNT: &str = "100000000000000000000";

#[tokio::main]
async fn main() {
    abigen!(Dai, "abis/Dai.json");

    let signer_middleware = get_signer_middleware().unwrap();

    let dai_address = DAI_ADDRESS.parse::<Address>().unwrap();
    let dai = Dai::new(dai_address, Arc::new(signer_middleware));

    let recipient_address = RECEIPIENT_ADDRESS.parse::<Address>().unwrap();
    let mint_amount = MINT_AMOUNT.parse::<U256>().unwrap();

    let call = dai.mint(recipient_address, mint_amount).legacy();

    // broadcast the transaction, log its hash, and wait for it to be confirmed
    call.send()
        .await
        .unwrap()
        .log_msg("minting! here's the tx hash")
        .await
        .unwrap();

    println!("done!");
}
