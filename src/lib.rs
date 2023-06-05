use std::error::Error;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::Wallet,
};

const RPC_URL: &str = "https://sphinx.shardeum.org";

/// Returns a `SignerMiddleware` using a `Wallet` created from a private key stored in a `PRIVATE_KEY`
/// environment variable.
pub fn get_signer_middleware(
) -> Result<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, Box<dyn Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
    let wallet = std::env::var("PRIVATE_KEY")?.parse::<Wallet<SigningKey>>()?;

    Ok(SignerMiddleware::new(provider, wallet))
}
