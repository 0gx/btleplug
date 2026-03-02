mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_read_counter_increments() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::COUNTER_READ,
    );

    let first = peripheral.read(&char).await.unwrap();
    let second = peripheral.read(&char).await.unwrap();

    // Counter is a little-endian u32, second read should be > first
    let first_val = u32::from_le_bytes(first[..4].try_into().unwrap());
    let second_val = u32::from_le_bytes(second[..4].try_into().unwrap());
    assert!(
        second_val > first_val,
        "Counter should increment: first={}, second={}",
        first_val,
        second_val
    );

    peripheral.disconnect().await.unwrap();
}
