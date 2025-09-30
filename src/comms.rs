use btleplug::platform::PeripheralId;

pub async fn do_connect(id: PeripheralId) -> Result<PeripheralId, btleplug::Error> {
    println!("Connecting to {}", id);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(id)
}

pub async fn do_disconnect(id: PeripheralId) -> Result<PeripheralId, btleplug::Error> {
    println!("Disconnecting from {}", id);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(id)
}
