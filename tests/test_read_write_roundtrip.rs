mod common;

use btleplug::api::{Peripheral as _, WriteType};

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_read_write_roundtrip() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::READ_WRITE,
    );

    let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
    peripheral
        .write(&char, &data, WriteType::WithResponse)
        .await
        .unwrap();

    let read_back = peripheral.read(&char).await.unwrap();
    assert_eq!(read_back, data, "Read-back should match written data");

    peripheral.disconnect().await.unwrap();
}
