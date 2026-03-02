mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_descriptor_discovery() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    // Find the Descriptor Test Char and verify it has our custom descriptors
    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::DESCRIPTOR_TEST_CHAR,
    );

    let descriptor_uuids: Vec<_> = char.descriptors.iter().map(|d| d.uuid).collect();

    // Should contain our custom descriptors (may also contain CCCD etc.)
    assert!(
        descriptor_uuids.contains(&common::gatt_uuids::READ_ONLY_DESCRIPTOR),
        "Read-only descriptor not found. Found: {:?}",
        descriptor_uuids
    );
    assert!(
        descriptor_uuids.contains(&common::gatt_uuids::READ_WRITE_DESCRIPTOR),
        "Read/write descriptor not found. Found: {:?}",
        descriptor_uuids
    );

    peripheral.disconnect().await.unwrap();
}
