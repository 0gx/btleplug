mod common;

use btleplug::api::{CharPropFlags, Peripheral as _};

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_characteristic_properties() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    let static_read = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::STATIC_READ,
    );
    assert!(
        static_read.properties.contains(CharPropFlags::READ),
        "Static Read should have READ property"
    );

    let write_char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::WRITE_WITH_RESPONSE,
    );
    assert!(
        write_char.properties.contains(CharPropFlags::WRITE),
        "Write With Response should have WRITE property"
    );

    let write_no_resp = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::WRITE_WITHOUT_RESPONSE,
    );
    assert!(
        write_no_resp
            .properties
            .contains(CharPropFlags::WRITE_WITHOUT_RESPONSE),
        "Write Without Response should have WRITE_WITHOUT_RESPONSE property"
    );

    peripheral.disconnect().await.unwrap();
}
