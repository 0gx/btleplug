mod common;

use btleplug::api::{Peripheral as _, WriteType};

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_write_with_response() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::WRITE_WITH_RESPONSE,
    );

    let data = vec![0xAA, 0xBB, 0xCC];
    // Should succeed without error (write-with-response gets acknowledgement)
    peripheral
        .write(&char, &data, WriteType::WithResponse)
        .await
        .unwrap();

    peripheral.disconnect().await.unwrap();
}
