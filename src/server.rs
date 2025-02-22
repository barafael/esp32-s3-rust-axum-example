use crate::wifi::WifiState;
use log::info;
use serde_json::{json, Value};
use std::sync::atomic::AtomicIsize;
use std::{net::SocketAddr, sync::Arc};

// Shared state that all Axum handlers can access
struct SharedState {
    pub counter: AtomicIsize,
    pub wifi_state: Arc<WifiState>,
}

// Starts the server. This function only returns in case of an error.
pub async fn run_server(wifi_state: Arc<WifiState>) -> anyhow::Result<()> {
    todo!()
}
