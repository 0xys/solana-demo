use solana_sdk::signature::{Signer};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_instruction::transfer;
use solana_sdk::message::Message;
use solana_sdk::transaction::Transaction;

use solana_client::rpc_client::RpcClient;

use std::str::FromStr;

mod account;
use account::{Account, FromBase58};

fn main() {
    let base58_str = "4DTgASfteRm3dJhJjbrbBqt5zagNVVw2G22ApLxCWzTy3VAi4M1kUUoetUWYfKMKtrxVty33jBUnHB7Me9ziKWK6";
    
    let account = Account::from_base58_str(base58_str);
    println!("from: {:?}", account.keypair.pubkey());

    let to = "EX1u9edUL77CoAZfPT9so9sUgoEmagjMc9zrussngAV6";

    faucet_account(to);
    
    // transfer_demo(&account, to, 123);
}

#[allow(dead_code)]
fn faucet_account(address: &str){
    let to = Pubkey::from_str(address).unwrap();
    let client = RpcClient::new(String::from("https://devnet.solana.com"));
    let signature = client.request_airdrop(&to, 1000000).unwrap();
    println!("faucet transaction: {:?}", signature);
}

#[allow(dead_code)]
fn transfer_demo(account: &Account, to: &str, amount: u64){
    let to = Pubkey::from_str(to).unwrap();

    let client = RpcClient::new(String::from("https://devnet.solana.com"));
    let (hash, _) = client.get_recent_blockhash().unwrap();
    println!("recent hash: {:?}", hash);

    let instruction = transfer(&account.keypair.pubkey(), &to, amount);
    let message = Message::new(&[instruction], Some(&account.keypair.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    
    println!("tx: {:?}", tx);
    
    tx.try_sign(&[&account.keypair], hash).unwrap();

    let signature = client.send_and_confirm_transaction(&tx).unwrap();

    println!("sig: {:?}", signature);
}