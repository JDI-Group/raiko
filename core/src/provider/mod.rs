use alloy_primitives::{Address, B256, U256};
use alloy_rpc_types::Block;
use raiko_lib::consts::{ChainSpec, SupportedChainSpecs};
use reth_primitives::{revm_primitives::AccountInfo, Header};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::{
    interfaces::{RaikoError, RaikoResult},
    provider::rpc::RpcBlockDataProvider,
    MerkleProof,
};

pub mod db;
pub mod rpc;

#[allow(async_fn_in_trait)]
pub trait BlockDataProvider {
    async fn get_blocks(&self, blocks_to_fetch: &[(u64, bool)]) -> RaikoResult<Vec<Block>>;

    async fn get_accounts(&self, accounts: &[Address]) -> RaikoResult<Vec<AccountInfo>>;

    async fn get_storage_values(&self, accounts: &[(Address, U256)]) -> RaikoResult<Vec<U256>>;

    async fn get_merkle_proofs(
        &self,
        block_number: u64,
        accounts: HashMap<Address, Vec<U256>>,
        offset: usize,
        num_storage_proofs: usize,
    ) -> RaikoResult<MerkleProof>;
}

pub async fn get_task_data(
    network: &str,
    block_number: u64,
    chain_specs: &SupportedChainSpecs,
) -> RaikoResult<(u64, B256)> {
    let taiko_chain_spec = chain_specs
        .get_chain_spec(network)
        .ok_or_else(|| RaikoError::InvalidRequestConfig("Unsupported raiko network".to_string()))?;
    let provider = RpcBlockDataProvider::new(&taiko_chain_spec.rpc.clone(), block_number - 1)?;
    let blocks = provider.get_blocks(&[(block_number, true)]).await?;
    let block = blocks
        .first()
        .ok_or_else(|| RaikoError::RPC("No block for requested block number".to_string()))?;
    let blockhash = block
        .header
        .hash
        .ok_or_else(|| RaikoError::RPC("No block hash for requested block".to_string()))?;
    Ok((taiko_chain_spec.chain_id, blockhash))
}

/// Check if the chain is BSC (BNB Smart Chain)
pub fn is_bsc_chain(chain_id: u64) -> bool {
    matches!(chain_id, 56 | 97)
}

/// Get block hash from RPC for BSC networks to ensure consistency with eth.getBlock()
pub async fn get_bsc_block_hash(
    chain_spec: &ChainSpec,
    block_number: u64,
) -> RaikoResult<B256> {
    debug!("Getting BSC block hash for block {block_number} from RPC");
    
    let provider = RpcBlockDataProvider::new(&chain_spec.rpc, 0)?;
    let blocks = provider.get_blocks(&[(block_number, false)]).await?;
    let block = blocks
        .first()
        .ok_or_else(|| RaikoError::RPC(format!("No block data for block {block_number}")))?;
    
    let block_hash = block
        .header
        .hash
        .ok_or_else(|| RaikoError::RPC(format!("No block hash for block {block_number}")))?;
    
    info!("Retrieved BSC block hash for block {block_number}: {block_hash:?}");
    Ok(block_hash)
}