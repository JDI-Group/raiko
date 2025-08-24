use std::str::FromStr;
use alloy_primitives::B256;
use serde_json::json;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // BSC testnet RPC endpoint
    let rpc_url = "https://bsc-testnet.blockpi.network/v1/rpc/a8cfdf19ef92e47b56498de208ed458e5e160f92";
    
    // The blob hash provided
    let blob_hash_str = "0x019101fb28118ceccaabca22a47e35b9c3f12eb2dcb25e5c543d5b75e6cd841f";
    let blob_hash = B256::from_str(blob_hash_str).unwrap();
    
    println!("Testing BSC blob retrieval with blob_hash: {}", blob_hash);
    
    // First, let's try to find transactions that might contain this blob
    // We'll need to search through recent blocks to find blob transactions
    let client = reqwest::Client::new();
    
    // Get the latest block number
    let latest_block_response = client
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    if let Some(latest_block_hex) = latest_block_response["result"].as_str() {
        let latest_block = u64::from_str_radix(latest_block_hex.trim_start_matches("0x"), 16)?;
        println!("Latest block: {}", latest_block);
        
        // Search through recent blocks for blob transactions
        for block_num in (latest_block.saturating_sub(100)..=latest_block).rev() {
            let block_response = client
                .post(rpc_url)
                .json(&json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByNumber",
                    "params": [format!("0x{:x}", block_num), true],
                    "id": 1
                }))
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;
            
            if let Some(block) = block_response["result"].as_object() {
                if let Some(transactions) = block["transactions"].as_array() {
                    for tx in transactions {
                        if let Some(tx_hash) = tx["hash"].as_str() {
                            if let Some(blob_hashes) = tx["blobVersionedHashes"].as_array() {
                                for blob_hash_val in blob_hashes {
                                    if let Some(blob_hash_in_tx) = blob_hash_val.as_str() {
                                        if blob_hash_in_tx == blob_hash_str {
                                            println!("Found transaction with matching blob hash!");
                                            println!("Block: {}, Transaction: {}", block_num, tx_hash);
                                            
                                            // Now try to get the blob sidecar using eth_getBlobSidecarByTxHash
                                            let sidecar_response = client
                                                .post(rpc_url)
                                                .json(&json!({
                                                    "jsonrpc": "2.0",
                                                    "method": "eth_getBlobSidecarByTxHash",
                                                    "params": [tx_hash, true],
                                                    "id": 1
                                                }))
                                                .send()
                                                .await?
                                                .json::<serde_json::Value>()
                                                .await?;
                                            
                                            println!("Sidecar response: {}", serde_json::to_string_pretty(&sidecar_response)?);
                                            return Ok(());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Add a small delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        println!("No transaction found with the specified blob hash in recent blocks");
    }
    
    Ok(())
}