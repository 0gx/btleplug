mod common;

use btleplug::api::Peripheral as _;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_peripheral_triggered_disconnect() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    assert!(peripheral.is_connected().await.unwrap());

    // Tell the peripheral to disconnect us after 500ms
    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_TRIGGER_DISCONNECT,
    )
    .await;

    // Wait for the disconnect to happen
    time::sleep(Duration::from_secs(2)).await;
    assert!(
        !peripheral.is_connected().await.unwrap(),
        "Peripheral should have disconnected us"
    );
}
