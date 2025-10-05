use anyhow::Context;
use meshtastic::api::{ConnectedStreamApi, StreamApi};
use meshtastic::packet::PacketReceiver;
use meshtastic::utils;
use meshtastic::utils::stream::BleId;
use std::time::Duration;

pub async fn do_connect(id: BleId) -> Result<(PacketReceiver, ConnectedStreamApi), anyhow::Error> {
    println!("Connecting to {}", id);
    let ble_stream = utils::stream::build_ble_stream(&id, Duration::from_secs(5)).await?;
    let stream_api = StreamApi::new();
    let (packet_receiver, stream_api) = stream_api.connect(ble_stream).await;
    let config_id = utils::generate_rand_id();
    let stream_api = stream_api.configure(config_id).await?;
    Ok((packet_receiver, stream_api))
}

pub async fn do_disconnect(
    id: BleId,
    stream_api: ConnectedStreamApi,
) -> Result<BleId, anyhow::Error> {
    println!("Disconnecting from {}", id);
    stream_api
        .disconnect()
        .await
        .context("Failed to disconnect")?;
    Ok(id)
}
