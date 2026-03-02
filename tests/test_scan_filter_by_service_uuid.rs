mod common;

use btleplug::api::{Central, ScanFilter};
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_scan_filter_by_service_uuid() {
    let adapter = common::peripheral_finder::get_adapter().await;

    // Scan with filter for our Control Service UUID
    adapter
        .start_scan(ScanFilter {
            services: vec![common::gatt_uuids::CONTROL_SERVICE],
        })
        .await
        .unwrap();

    time::sleep(Duration::from_secs(5)).await;

    let peripherals = adapter.peripherals().await.unwrap();
    adapter.stop_scan().await.unwrap();

    // At least one peripheral should match (our test peripheral)
    assert!(
        !peripherals.is_empty(),
        "No peripherals found with Control Service UUID filter"
    );
}
