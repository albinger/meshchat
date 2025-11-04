use iced::futures::{SinkExt, Stream};
use iced::stream;
use meshtastic::utils::stream::{available_ble_devices, BleDevice};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum DiscoveryEvent {
    BLERadioFound(BleDevice),
    BLERadioLost(BleDevice),
    Error(String),
}

/// A stream of [DiscoveryEvent] announcing the discovery or loss of devices via BLE
pub fn ble_discovery() -> impl Stream<Item = DiscoveryEvent> {
    stream::channel(100, move |mut gui_sender| async move {
        let mut mesh_radio_ids: Vec<BleDevice> = vec![];

        // loop scanning for devices
        loop {
            match available_ble_devices(Duration::from_secs(4)).await {
                Ok(radios_now_ids) => {
                    // detect lost radios
                    for id in &mesh_radio_ids {
                        if !radios_now_ids.iter().any(|other_id| id == other_id) {
                            // inform GUI of a device lost
                            gui_sender
                                .send(DiscoveryEvent::BLERadioLost(id.clone()))
                                .await
                                .unwrap_or_else(|e| eprintln!("Discovery gui send error: {e}"));
                        }
                    }

                    // detect new radios found
                    for id in &radios_now_ids {
                        if !mesh_radio_ids.iter().any(|other_id| id == other_id) {
                            // track it for the future
                            mesh_radio_ids.push(id.clone());

                            // inform GUI of a new device found
                            gui_sender
                                .send(DiscoveryEvent::BLERadioFound(id.clone()))
                                .await
                                .unwrap_or_else(|e| eprintln!("Discovery gui send error: {e}"));
                        }
                    }
                }
                Err(e) => {
                    gui_sender
                        .send(DiscoveryEvent::Error(e.to_string()))
                        .await
                        .unwrap_or_else(|e| eprintln!("Discovery gui send error: {e}"));
                }
            }
        }
    })
}
