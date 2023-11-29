use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    abigen!(ERC20Mintable, "./ERC20Mintable.abi.json");

    // Set up the provider, wallet, and client
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let wallet = "your-private-key".parse::<LocalWallet>()?;
    let client = SignerMiddleware::new(provider, wallet);

    // Instantiate the contract
    let erc20_mintable = ERC20Mintable::deploy(
        client,
        (
            "Base".to_string(),
            "BASE".to_string(),
            18_u8,
            Address::zero(),
            false,
        ),
    );

    // Interact with the contract
    let total_supply = erc20_mintable.total_supply().call().await?;

    println!("Total Supply: {}", total_supply);

    Ok(())
}
