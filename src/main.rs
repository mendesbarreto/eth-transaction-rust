extern crate web3;

use std::borrow::Borrow;
use web3::futures::Future;
use web3::types::{Address, TransactionRequest, U256};


#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:7545")?;
    let web3 = web3::Web3::new(transport);

    println!("Get all accounts available");
    let mut accounts = web3.eth().accounts().await?;

    // println!("print account names and balances");
    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!("Account name: {:?} balance: {:?}", account, balance);
    // }

    let stonks_address = accounts[0];
    let pepe_the_frog_address = accounts[1];

    println!("Pepe the Frog will send 1 ETH to Stonks");

    let value: i64 = 1000000000000000000;
    let transaction = TransactionRequest {
        from: pepe_the_frog_address,
        to: Some(stonks_address),
        gas: None,
        gas_price: None,
        value: Some(U256::from(value)),
        data: None,
        nonce: None,
        condition: None,
        transaction_type: None,
        access_list: None
    };

    let transaction_hash = web3.eth().send_transaction(transaction).await?;

    println!("Transaction completed!");


    let stonks_balance = web3.eth().balance(stonks_address, None).await?;
    let pepe_the_frog_balance = web3.eth().balance(pepe_the_frog_address, None).await?;

    println!("Stonks balance: {}", stonks_balance);
    println!("Pepe the Frog balance: {}", pepe_the_frog_balance);

    Ok(())
}
