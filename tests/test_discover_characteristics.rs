mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_discover_characteristics() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    let chars = peripheral.characteristics();
    let char_uuids: Vec<_> = chars.iter().map(|c| c.uuid).collect();

    // Spot-check key characteristics exist
    assert!(char_uuids.contains(&common::gatt_uuids::CONTROL_POINT));
    assert!(char_uuids.contains(&common::gatt_uuids::STATIC_READ));
    assert!(char_uuids.contains(&common::gatt_uuids::NOTIFY_CHAR));
    assert!(char_uuids.contains(&common::gatt_uuids::DESCRIPTOR_TEST_CHAR));

    peripheral.disconnect().await.unwrap();
}
