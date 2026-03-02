mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_properties_contain_peripheral_info() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    let props = peripheral
        .properties()
        .await
        .unwrap()
        .expect("properties should be available");

    // Should have a local name
    let expected_name = std::env::var("BTLEPLUG_TEST_PERIPHERAL")
        .unwrap_or_else(|_| common::gatt_uuids::TEST_PERIPHERAL_NAME.to_string());
    assert_eq!(
        props.local_name.as_deref(),
        Some(expected_name.as_str()),
    );

    // Should have manufacturer data
    assert!(
        props
            .manufacturer_data
            .contains_key(&common::gatt_uuids::MANUFACTURER_COMPANY_ID),
        "Properties should contain manufacturer data with company ID 0xFFFF"
    );

    // RSSI should be present from scan
    assert!(props.rssi.is_some(), "RSSI from scan should be present");

    peripheral.disconnect().await.unwrap();
}
