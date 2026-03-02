mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_mtu_after_connection() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    let mtu = peripheral.mtu();
    // After connection, MTU should be at least the default BLE MTU (23)
    // and potentially higher if MTU exchange succeeded
    assert!(
        mtu >= 23,
        "MTU should be at least 23 (default), got {}",
        mtu
    );

    peripheral.disconnect().await.unwrap();
}
