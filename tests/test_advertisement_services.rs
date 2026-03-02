mod common;

use btleplug::api::{Central, CentralEvent};
use btleplug::api::ScanFilter;
use futures::StreamExt;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_advertisement_services() {
    let adapter = common::peripheral_finder::get_adapter().await;
    let mut events = adapter.events().await.unwrap();

    adapter
        .start_scan(ScanFilter::default())
        .await
        .unwrap();

    let mut found_services = false;
    let timeout = time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(event) = events.next() => {
                if let CentralEvent::ServicesAdvertisement { services, .. } = event {
                    if services.contains(&common::gatt_uuids::CONTROL_SERVICE) {
                        found_services = true;
                        break;
                    }
                }
            }
            _ = &mut timeout => break,
        }
    }

    adapter.stop_scan().await.unwrap();
    assert!(
        found_services,
        "Did not receive ServicesAdvertisement with Control Service UUID"
    );
}
