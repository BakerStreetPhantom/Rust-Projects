use std::env;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

fn get_valid_timestamp(future_millis: u128) -> u128 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let time_millis = since_epoch.as_millis().checked_add(future_millis).unwrap();
    time_millis
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    for account in &accounts {
        let balance = web3s.eth().balance(*account, None).await?;
        println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    }

    let router02_addr = Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap();
    let router02_contract = Contract::from_json(
        web3s.eth(),
        router02_addr,
        include_bytes!("router02_abi.json"),
    )
    .unwrap();

    let weth_addr: Address = router02_contract
        .query("WETH", (), None, Options::default(), None)
        .await
        .unwrap();

    println!("WETH address: {:?}", &weth_addr);

    let usdc_address = Address::from_str("0xc7AD46e0b8a400Bb3C915120d284AafbA8fc4735").unwrap();
    let valid_timestamp = get_valid_timestamp(300000);
    println!("timemillis: {}", valid_timestamp);
    let out_gas_estimate = router02_contract
        .estimate_gas(
            "swapExactETHForTokens",
            (
                U256::from_dec_str("106662000000").unwrap(),
                vec![weth_addr, usdc_address],
                accounts[0],
                U256::from_dec_str(&valid_timestamp.to_string()).unwrap(),
            ),
            accounts[0],
            Options {
                value: Some(U256::exp10(18).checked_div(20.into()).unwrap()),
                gas: Some(500_000.into()),
                ..Default::default()
            },
        )
        .await
        .expect("Error");
    println!("estimated gas amount: {}", out_gas_estimate);

    let gas_price = web3s.eth().gas_price().await.unwrap();
    println!("gas price: {}", gas_price);
    Ok(())
}
