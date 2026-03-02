mod common;

use btleplug::api::{Peripheral as _, WriteType};

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_long_value_read_write() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::LONG_VALUE,
    );

    // Write a value larger than the default MTU (23 bytes)
    let data: Vec<u8> = (0..200).map(|i| (i % 256) as u8).collect();
    peripheral
        .write(&char, &data, WriteType::WithResponse)
        .await
        .unwrap();

    let read_back = peripheral.read(&char).await.unwrap();
    assert_eq!(
        read_back, data,
        "Long value read-back should match written data"
    );

    peripheral.disconnect().await.unwrap();
}
