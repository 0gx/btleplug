mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_discover_services() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    let services = peripheral.services();

    // Should have at least our 4 test services
    let service_uuids: Vec<_> = services.iter().map(|s| s.uuid).collect();
    assert!(
        service_uuids.contains(&common::gatt_uuids::CONTROL_SERVICE),
        "Control Service not found in {:?}",
        service_uuids
    );
    assert!(
        service_uuids.contains(&common::gatt_uuids::READ_WRITE_SERVICE),
        "Read/Write Service not found"
    );
    assert!(
        service_uuids.contains(&common::gatt_uuids::NOTIFICATION_SERVICE),
        "Notification Service not found"
    );
    assert!(
        service_uuids.contains(&common::gatt_uuids::DESCRIPTOR_SERVICE),
        "Descriptor Service not found"
    );

    peripheral.disconnect().await.unwrap();
}
