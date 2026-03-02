mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_read_write_descriptor_roundtrip() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let descriptor = common::find_descriptor(
        &peripheral,
        common::gatt_uuids::DESCRIPTOR_TEST_CHAR,
        common::gatt_uuids::READ_WRITE_DESCRIPTOR,
    );

    let data = vec![0x42, 0x43, 0x44];
    peripheral
        .write_descriptor(&descriptor, &data)
        .await
        .unwrap();

    let read_back = peripheral.read_descriptor(&descriptor).await.unwrap();
    assert_eq!(
        read_back, data,
        "Descriptor read-back should match written data"
    );

    peripheral.disconnect().await.unwrap();
}
