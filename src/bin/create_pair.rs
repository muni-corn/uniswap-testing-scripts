use std::sync::Arc;

use ethers::{prelude::{abigen, SignerMiddleware, k256::ecdsa::SigningKey}, providers::{Provider, Http, Middleware}, abi::Address, signers::{Wallet, Signer}};

const CONTRACT_ADDRESS: &str = "0x6CfbD6e190fEf2ab0Af7a9D17671F04B63F61F0e";
const RPC_URL: &str = "https://sphinx.shardeum.org";

/// WETH
const TOKEN_A: &str = "0x21bAC26E41FDf0b4fb73D8c9653C1821FEbd77b8";

/// DAI
const TOKEN_B: &str = "0x559787297A9E3E558C764a647c7877d84DAE1c05";

#[tokio::main]
async fn main() {
    abigen!(UniswapV2Factory, "abis/UniswapV2Factory.json");

    let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
    let wallet = std::env::var("PRIVATE_KEY").unwrap().parse::<Wallet<SigningKey>>().unwrap();

    let tx_count = provider.get_transaction_count(wallet.address(), None).await.unwrap();

    let signer_middleware = SignerMiddleware::new(provider, wallet);

    let contract_address = CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let factory = UniswapV2Factory::new(contract_address, Arc::new(signer_middleware));

    let token_a = TOKEN_A.parse::<Address>().unwrap();
    let token_b = TOKEN_B.parse::<Address>().unwrap();

    let call = factory.create_pair(token_a, token_b).legacy().nonce(tx_count);
    call.send().await.unwrap().log_msg("creating pair! here's the tx hash").await.unwrap();
}
