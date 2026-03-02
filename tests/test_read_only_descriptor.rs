mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_read_only_descriptor() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let descriptor = common::find_descriptor(
        &peripheral,
        common::gatt_uuids::DESCRIPTOR_TEST_CHAR,
        common::gatt_uuids::READ_ONLY_DESCRIPTOR,
    );

    let value = peripheral.read_descriptor(&descriptor).await.unwrap();
    assert_eq!(
        value,
        vec![0xDE, 0xAD, 0xBE, 0xEF],
        "Read-only descriptor should return fixed value"
    );

    peripheral.disconnect().await.unwrap();
}
