mod common;

use btleplug::api::{Peripheral as _, WriteType};

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_write_without_response() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::WRITE_WITHOUT_RESPONSE,
    );

    let data = vec![0x11, 0x22, 0x33];
    peripheral
        .write(&char, &data, WriteType::WithoutResponse)
        .await
        .unwrap();

    peripheral.disconnect().await.unwrap();
}
