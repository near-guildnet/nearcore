//! Indexer creates a queue, starts the near-streamer, passing this queue in there.
//! Listens to that queue and returns near_streamer::BlockResponse for further handling
use actix::System;
use neard;
use std::path::PathBuf;
mod streamer;

/// Indexer struct which have to be used for starting indexer
pub struct Indexer {
    home_dir: PathBuf,
    near_config: neard::config::NearConfig,
}

impl Indexer {
    /// Build the Indexer struct
    pub fn new() -> Self {
        let home_dir = PathBuf::from(neard::get_default_home());
        let near_config = neard::load_config(&home_dir);
        Self { home_dir, near_config }
    }

    /// Starts `actix::SystemRunner` with spawned nearcore and indexer.
    ///
    /// Checks genesis before starting and panics if invalid.
    pub fn start(&self) {
        neard::genesis_validate::validate_genesis(&self.near_config.genesis);
        let system = System::new("NEAR Indexer");
        let (client, view_client) =
            neard::start_with_config(&self.home_dir, self.near_config.clone());
        actix::spawn(streamer::start(client.clone(), view_client.clone()));
        system.run().unwrap();
    }
}
