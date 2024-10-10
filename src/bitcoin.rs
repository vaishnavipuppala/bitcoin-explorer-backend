use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;

pub fn get_client() -> Result<Client, bitcoincore_rpc::Error> {
    let rpc_url = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set in .env");
    let rpc_user = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set in .env");
    let rpc_password = env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set in .env");

    println!("Attempting to connect to Bitcoin node at URL: {}", rpc_url);
    
    Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).map_err(|e| {
        eprintln!("Failed to create Bitcoin RPC client: {:?}", e);
        e
    })
}

pub fn fetch_block_data(client: &Client) -> Result<(i64, f64), bitcoincore_rpc::Error> {
    println!("Attempting to fetch blockchain info...");
    let blockchain_info = client.get_blockchain_info()?;
    let block_height = blockchain_info.blocks as i64;

    println!("Fetching latest block hash...");
    let block_hash = client.get_block_hash(block_height as u64)?;

    println!("Fetching block data...");
    let block = client.get_block(&block_hash)?;

    let mut total_volume = 0.0;
    for tx in block.txdata {
        for output in tx.output {
            total_volume += output.value as f64 / 100_000_000.0; // Convert satoshis to BTC
        }
    }

    println!("Block height: {}", block_height);
    println!("Total transaction volume: {} BTC", total_volume);

    Ok((block_height, total_volume))
}