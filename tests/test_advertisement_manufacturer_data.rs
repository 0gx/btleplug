mod common;

use btleplug::api::{Central, CentralEvent};
use btleplug::api::ScanFilter;
use futures::StreamExt;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_advertisement_manufacturer_data() {
    let adapter = common::peripheral_finder::get_adapter().await;
    let mut events = adapter.events().await.unwrap();

    adapter
        .start_scan(ScanFilter::default())
        .await
        .unwrap();

    let mut found_manufacturer_data = false;
    let timeout = time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(event) = events.next() => {
                if let CentralEvent::ManufacturerDataAdvertisement { manufacturer_data, .. } = event {
                    if manufacturer_data.contains_key(&common::gatt_uuids::MANUFACTURER_COMPANY_ID) {
                        found_manufacturer_data = true;
                        break;
                    }
                }
            }
            _ = &mut timeout => break,
        }
    }

    adapter.stop_scan().await.unwrap();
    assert!(
        found_manufacturer_data,
        "Did not receive ManufacturerDataAdvertisement with company ID 0xFFFF"
    );
}
