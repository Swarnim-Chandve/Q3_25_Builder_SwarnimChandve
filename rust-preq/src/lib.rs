#[cfg(test)]
mod tests{
 use bs58;
use std::io::{self, BufRead};
use std::str::FromStr;

use solana_client::rpc_client::RpcClient;

use solana_program::{pubkey::Pubkey, system_instruction::transfer, hash::hash};

use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};

const RPC_URL: &str =
    "https://api.devnet.solana.com";


     #[test]
    fn transfer_sol() {
          let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    let pubkey = keypair.pubkey();

    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    }

    let to_pubkey = Pubkey::from_str("A5coRkQDycN8W7vdKK383iP2zfTxhLKdkuM625g3ixjp").unwrap();

    let rpc_client = RpcClient::new(RPC_URL);

    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    }
}
