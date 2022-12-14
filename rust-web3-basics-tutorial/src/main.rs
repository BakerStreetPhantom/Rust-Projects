use std::env;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    }
    
    let aave_addr = Address::from_str("0x42447d5f59d5bf78a82c34663474922bdf278162").unwrap();
    let token_contract =
        Contract::from_json(web3s.eth(), aave_addr, include_bytes!("erc20_abi.json")).unwrap();
    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();
    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();
    println!("Token name: {}, total supply: {}", token_name, total_supply);

    Ok(())
}