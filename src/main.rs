use solana_sdk::signature::{Keypair,Signer};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_instruction::transfer;
use solana_sdk::message::Message;
use solana_sdk::transaction::Transaction;

use solana_client::rpc_client::RpcClient;

use std::str::FromStr;

fn main() {
    let base58_str = "4DTgASfteRm3dJhJjbrbBqt5zagNVVw2G22ApLxCWzTy3VAi4M1kUUoetUWYfKMKtrxVty33jBUnHB7Me9ziKWK6";
    let kp = Keypair::from_base58_string(base58_str);
    println!("from: {:?}", kp.pubkey());

    let to = "EX1u9edUL77CoAZfPT9so9sUgoEmagjMc9zrussngAV6";
    let to = Pubkey::from_str(to).unwrap();

    let client = RpcClient::new(String::from("https://devnet.solana.com"));
    let (hash, _) = client.get_recent_blockhash().unwrap();
    println!("recent hash: {:?}", hash);

    let instruction = transfer(&kp.pubkey(), &to, 123);
    let message = Message::new(&[instruction], Some(&kp.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    
    println!("tx: {:?}", tx);
    
    tx.try_sign(&[&kp], hash).unwrap();

    let signature = client.send_and_confirm_transaction(&tx).unwrap();

    println!("sig: {:?}", signature);
}
