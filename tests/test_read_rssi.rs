mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_read_rssi() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    match peripheral.read_rssi().await {
        Ok(rssi) => {
            // RSSI should be a negative dBm value (typically -30 to -100)
            assert!(
                rssi < 0 && rssi > -120,
                "RSSI should be between -120 and 0 dBm, got {}",
                rssi
            );
        }
        Err(btleplug::Error::NotSupported(_)) => {
            // Some platforms may not support read_rssi — acceptable
        }
        Err(e) => {
            panic!("Unexpected error from read_rssi: {:?}", e);
        }
    }

    peripheral.disconnect().await.unwrap();
}
