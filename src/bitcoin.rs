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

pub fn fetch_block_data(client: &Client) -> Result<(i64, f64, usize, f64, f64, f64, f64, i64), bitcoincore_rpc::Error> {
    println!("Attempting to fetch blockchain info...");
    let blockchain_info = client.get_blockchain_info()?;
    let block_height = blockchain_info.blocks as i64;
    let difficulty = blockchain_info.difficulty;

    println!("Fetching latest block hash...");
    let block_hash = client.get_block_hash(block_height as u64)?;

    println!("Fetching block data...");
    let block = client.get_block(&block_hash)?;

    let mut total_volume = 0.0;
    let transaction_count = block.txdata.len();
    let block_size = block.size() as f64 / 1_000_000.0; // Convert to MB
    let mut total_fees = 0.0;

    for (tx_index, tx) in block.txdata.iter().enumerate() {
        for output in &tx.output {
            total_volume += satoshi_to_btc(output.value);
        }

        // Calculate transaction fees more accurately
        if !tx.is_coin_base() {
            let inputs_value: u64 = tx.input
                .iter()
                .filter_map(|input| {
                    match client.get_tx_out(&input.previous_output.txid, input.previous_output.vout, Some(true)) {
                        Ok(Some(txout)) => Some(txout.value.to_sat()),
                        Ok(None) => {
                            println!("Warning: Couldn't find output for input in tx {}", tx.txid());
                            None
                        },
                        Err(e) => {
                            println!("Error fetching tx_out for tx {}: {:?}", tx.txid(), e);
                            None
                        }
                    }
                })
                .sum();

            let outputs_value: u64 = tx.output
                .iter()
                .map(|output| output.value)
                .sum();

            let tx_fee = inputs_value.saturating_sub(outputs_value);
            println!("Transaction {} (index {}): Inputs = {} satoshis, Outputs = {} satoshis, Fee = {} satoshis", 
                     tx.txid(), tx_index, inputs_value, outputs_value, tx_fee);
            total_fees += satoshi_to_btc(tx_fee);
        }
    }

    // Fetch network hashrate (in exahashes per second)
    let network_hashps = client.get_network_hash_ps(Some(144), Some(block_height as u64))? / 1e18;

    // Fetch mempool info
    let mempool_size = client.get_raw_mempool()?.len() as i64;

    println!("Block height: {}", block_height);
    println!("Total transaction volume: {:.8} BTC", total_volume);
    println!("Transaction count: {}", transaction_count);
    println!("Block size: {:.2} MB", block_size);
    println!("Total fees: {:.8} BTC", total_fees);
    println!("Difficulty: {}", difficulty);
    println!("Network hashrate: {:.2} EH/s", network_hashps);
    println!("Mempool size: {} transactions", mempool_size);

    // Sanity check for zero fees
    if total_fees == 0.0 && transaction_count > 1 {
        println!("Warning: Total fees are zero for a block with {} transactions. This is highly unusual.", transaction_count);
    }

    Ok((block_height, total_volume, transaction_count, block_size, total_fees, difficulty, network_hashps, mempool_size))
}

// Helper function to convert satoshis to BTC
fn satoshi_to_btc(satoshi: u64) -> f64 {
    satoshi as f64 / 100_000_000.0
}