use faic_node::{
    core::{interfaces::*, error::*, logger::*},
    network::*,
    wallet::*,
};

#[tokio::main]
async fn main() -> Result<(), FAICError> {
    // Initialize logger
    let logger = Logger::new("main".to_string(), "node".to_string());
    logger.logInfo("Starting FAIC Node...");

    // TODO: Initialize other components

    Ok(())
}