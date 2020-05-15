//! Streamer watches the network and collects all the blocks and related chunks
//! into one struct and pushes in in to the given queue
use actix::Addr;
use futures;
use futures::stream::StreamExt;
use log::{debug, info};
use near_client;
use near_primitives::{types, views};
use std::time::Duration;
use tokio::time;

const INTERVAL: Duration = Duration::from_millis(500);
const INDEXER: &str = "indexer";

/// Error occurs in case of failed data fetch
#[derive(Debug)]
pub struct FailedToFetchData;

/// Resulting struct represents block with chunks
#[derive(Debug)]
pub struct BlockResponse {
    block: views::BlockView,
    chunks: Vec<views::ChunkView>,
}

/// Fetches the status to retrieve `latest_block_height` to determine if we need to fetch
/// entire block or we already fetched this block.
async fn poll_status(
    client: &Addr<near_client::ClientActor>,
) -> Result<near_client::StatusResponse, String> {
    client
        .send(near_client::Status { is_health_check: false })
        .await
        .map_err(|err| err.to_string())?
}

/// This function supposed to return the entire `BlockResponse`.
/// It calls fetches the block and fetches all the chunks for the block
/// and returns everything together in one struct
async fn fetch_block(
    client: &Addr<near_client::ViewClientActor>,
    block_height: u64,
) -> Result<BlockResponse, FailedToFetchData> {
    let block_id_or_finality =
        types::BlockIdOrFinality::BlockId(types::BlockId::Height(block_height));
    let block = client
        .send(near_client::GetBlock(block_id_or_finality))
        .await
        .map_err(|_| FailedToFetchData)?
        .map_err(|_| FailedToFetchData)?;
    let chunks = fetch_chunks(&client, &block.chunks).await?;
    Ok(BlockResponse { block, chunks })
}

/// Fetches single chunk (as `near_primitives::views::ChunkView`) by provided `near_client::GetChunk` enum
async fn fetch_single_chunk(
    client: &Addr<near_client::ViewClientActor>,
    get_chunk: near_client::GetChunk,
) -> Result<views::ChunkView, FailedToFetchData> {
    client.send(get_chunk).await.map_err(|_| FailedToFetchData)?.map_err(|_| FailedToFetchData)
}

/// Fetches all the chunks by their hashes and returns them as a `Vec`
async fn fetch_chunks(
    client: &Addr<near_client::ViewClientActor>,
    chunks: &[views::ChunkHeaderView],
) -> Result<Vec<views::ChunkView>, FailedToFetchData> {
    let chunks_hashes =
        chunks.iter().map(|chunk| near_client::GetChunk::ChunkHash(chunk.chunk_hash.into()));
    let mut chunks: futures::stream::FuturesUnordered<_> =
        chunks_hashes.map(|get_chunk| fetch_single_chunk(&client, get_chunk)).collect();
    let mut response: Vec<views::ChunkView> = vec![];
    while let Some(chunk) = chunks.next().await {
        response.push(chunk?);
    }

    Ok(response)
}

/// Function that starts Streamer's busy loop. Every half a seconds it fetches the status
/// compares to already fetched block height and in case it differs fetches new block of given height.
///
/// We have to pass `client: Addr<near_client::ClientActor>` and `view_client: Addr<near_client::ViewClientActor>`.
pub async fn start(
    client: Addr<near_client::ClientActor>,
    view_client: Addr<near_client::ViewClientActor>,
) {
    info!(target: INDEXER, "Starting Streamer...");
    let mut last_fetched_block_height: types::BlockHeight = 0;
    loop {
        time::delay_for(INTERVAL).await;
        match poll_status(&client).await {
            Ok(status_response) => {
                info!(
                    target: INDEXER,
                    "Last block height is {}", status_response.sync_info.latest_block_height
                );
                let latest_block_height = status_response.sync_info.latest_block_height;
                if latest_block_height > last_fetched_block_height {
                    last_fetched_block_height = latest_block_height;
                    info!(target: INDEXER, "The block is new");
                    let block_response =
                        fetch_block(&view_client, latest_block_height).await.unwrap();
                    debug!(target: INDEXER, "{:#?}", block_response);
                }
            }
            _ => {}
        };
    }
}
