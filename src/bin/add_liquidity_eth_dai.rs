use std::sync::Arc;

use ethers::{
    abi::Address,
    prelude::abigen,
};
use uniswap_testing::get_signer_middleware;

const ROUTER_CONTRACT_ADDRESS: &str = "0xeed4ef0Fc40c3B13a9f85eDcf1879609281BCA94";
const DAI_ADDRESS: &str = "0x559787297A9E3E558C764a647c7877d84DAE1c05";
const AMOUNT_TOKEN_DESIRED: &str = "1000000000000000000000";
const AMOUNT_TOKEN_MIN: &str = "100000000000000000000";
const AMOUNT_ETH_MIN: &str = "100000000000000000";
const RECIPIENT_ADDRESS: &str = "0xAE2c4A62737E02973Cc2955d5e98264EcDc4Aa12";
const DEADLINE: &str = "10000000000000000000000000000";

#[tokio::main]
async fn main() {
    abigen!(UniswapV2Router02, "abis/UniswapV2Router02.json");

    let signer_middleware = get_signer_middleware().unwrap();

    let contract_address = ROUTER_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let recipient_address = RECIPIENT_ADDRESS.parse::<Address>().unwrap();
    let dai = DAI_ADDRESS.parse::<Address>().unwrap();

    let router = UniswapV2Router02::new(contract_address, Arc::new(signer_middleware));

    let call = router.add_liquidity_eth(
        dai,
        AMOUNT_TOKEN_DESIRED.into(),
        AMOUNT_TOKEN_MIN.into(),
        AMOUNT_ETH_MIN.into(),
        recipient_address,
        DEADLINE.into(),
    ).value("200000000000000000").legacy();
    call.send()
        .await
        .unwrap()
        .log_msg("adding liquidity! here's the tx hash")
        .await
        .unwrap();

    println!("done!");
}


