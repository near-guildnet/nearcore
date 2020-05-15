//! Streamer watches the network and collects all the blocks and related chunks
//! into one struct and pushes in in to the given queue
use std::time::Duration;

use actix::Addr;
use futures::stream::StreamExt;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, info};

use near_client;
use near_primitives::{types, views};

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
async fn fetch_latest_block(
    client: &Addr<near_client::ViewClientActor>,
) -> Result<views::BlockView, FailedToFetchData> {
    client
        .send(near_client::GetBlock::latest())
        .await
        .map_err(|_| FailedToFetchData)?
        .map_err(|_| FailedToFetchData)
}

/// This function supposed to return the entire `BlockResponse`.
/// It calls fetches the block and fetches all the chunks for the block
/// and returns everything together in one struct
async fn fetch_block_with_chunks(
    client: &Addr<near_client::ViewClientActor>,
    block: views::BlockView,
) -> Result<BlockResponse, FailedToFetchData> {
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
    view_client: Addr<near_client::ViewClientActor>,
    mut queue: mpsc::Sender<BlockResponse>,
) {
    info!(target: INDEXER, "Starting Streamer...");
    let mut last_fetched_block_height: types::BlockHeight = 0;
    loop {
        time::delay_for(INTERVAL).await;
        match fetch_latest_block(&view_client).await {
            Ok(block) => {
                let latest_block_height = block.header.height;
                if latest_block_height > last_fetched_block_height {
                    last_fetched_block_height = latest_block_height;
                    info!(target: INDEXER, "The block is new");
                    let block_response =
                        fetch_block_with_chunks(&view_client, block).await.unwrap();
                    debug!(target: INDEXER, "{:#?}", &block_response);
                    queue.send(block_response).await;
                }
            }
            _ => {}
        };
    }
}
