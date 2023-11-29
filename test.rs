// Use `abigen!` macro to generate bindings for ERC20Mintable contract
abigen!(ERC20Mintable, "./ERC20Mintable.abi.json");

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test as tokio_test;
    use std::{convert::TryFrom, sync::Arc, time::Duration};
    use ethers::{
        core::utils::Anvil,
        middleware::SignerMiddleware,
        providers::{Http, Middleware, Provider},
        signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
        types::{Address, Bytes, U256},
        utils::AnvilInstance,
    };

    #[tokio_test]
    async fn test_erc20_mintable_deployment() -> Result<(), Box<dyn std::error::Error>> {
        let anvil = Anvil::new()
            .arg("--code-size-limit")
            .arg("120000")
            .arg("--disable-block-gas-limit")
            .spawn();
        
        let provider = Provider::<Http>::try_from(anvil.endpoint())?;
        let client = ChainClient::new(provider);
        let mut contract_deployment_tx = ERC20Mintable::deploy(
            client.clone(),
            (
                "Base".to_string(),
                "BASE".to_string(),
                18_u8,
                Address::zero(),
                false,
            ),
        );
        contract_deployment_tx.as_mut().unwrap().deployer.tx.set_gas_price(1_000_000_000);
        let base = contract_deployment_tx?
        .send()
        .await?;

        Ok(())
    }
}