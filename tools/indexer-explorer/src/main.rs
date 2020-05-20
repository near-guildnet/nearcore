use std::env;
use std::io;

use actix;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
#[macro_use]
extern crate diesel;
use tokio::sync::mpsc;
use tokio_diesel::*;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

use near_indexer;

mod models;

mod schema;

fn init_logging(verbose: bool) {
    let mut env_filter = EnvFilter::new("tokio_reactor=info,near=info,stats=info,telemetry=info");

    if verbose {
        env_filter = env_filter
            .add_directive("cranelift_codegen=warn".parse().unwrap())
            .add_directive("cranelift_codegen=warn".parse().unwrap())
            .add_directive("h2=warn".parse().unwrap())
            .add_directive("trust_dns_resolver=warn".parse().unwrap())
            .add_directive("trust_dns_proto=warn".parse().unwrap());

        env_filter = env_filter.add_directive(LevelFilter::DEBUG.into());
    } else {
        env_filter = env_filter.add_directive(LevelFilter::WARN.into());
    }

    if let Ok(rust_log) = env::var("RUST_LOG") {
        for directive in rust_log.split(',').filter_map(|s| match s.parse() {
            Ok(directive) => Some(directive),
            Err(err) => {
                eprintln!("Ignoring directive `{}`: {}", s, err);
                None
            }
        }) {
            env_filter = env_filter.add_directive(directive);
        }
    }
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(io::stderr)
        .init();
}

async fn listen_blocks(mut stream: mpsc::Receiver<near_indexer::BlockResponse>) {
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://near:1111@localhost/near_indexer");
    let pool = Pool::builder().build(manager).unwrap();

    while let Some(block) = stream.recv().await {
        // TODO: handle data as you need
        info!(target: "stats", "Block height {}", &block.block.header.height);
        match diesel::insert_into(schema::blocks::table)
            .values(models::Block::from_block_view(&block.block))
            .execute_async(&pool)
            .await
        {
            Ok(_) => {}
            Err(_) => continue,
        };

        diesel::insert_into(schema::chunks::table)
            .values(
                block
                    .chunks
                    .iter()
                    .map(|chunk| models::Chunk::from_chunk_view(block.block.header.height, chunk))
                    .collect::<Vec<models::Chunk>>(),
            )
            .execute_async(&pool)
            .await
            .unwrap();
    }
}

fn main() {
    init_logging(false);
    let indexer = near_indexer::Indexer::new();
    let stream = indexer.receiver();
    actix::spawn(listen_blocks(stream));
    indexer.start();
}
