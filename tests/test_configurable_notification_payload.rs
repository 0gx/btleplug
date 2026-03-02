mod common;

use btleplug::api::Peripheral as _;
use futures::StreamExt;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_configurable_notification_payload() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let config_char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::CONFIGURABLE_NOTIFY,
    );
    let control_point = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::CONTROL_POINT,
    );

    // Set a custom payload via control point: opcode 0x06 + payload bytes
    let mut cmd = vec![common::gatt_uuids::CMD_SET_NOTIFICATION_PAYLOAD];
    cmd.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);
    peripheral
        .write(&control_point, &cmd, btleplug::api::WriteType::WithResponse)
        .await
        .unwrap();

    let mut stream = peripheral.notifications().await.unwrap();
    peripheral.subscribe(&config_char).await.unwrap();

    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_START_NOTIFICATIONS,
    )
    .await;

    // Wait for a notification with our custom payload
    let timeout = time::sleep(Duration::from_secs(5));
    tokio::pin!(timeout);
    let mut matching = false;

    loop {
        tokio::select! {
            Some(n) = stream.next() => {
                if n.uuid == common::gatt_uuids::CONFIGURABLE_NOTIFY
                    && n.value == vec![0xCA, 0xFE, 0xBA, 0xBE]
                {
                    matching = true;
                    break;
                }
            }
            _ = &mut timeout => break,
        }
    }

    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_STOP_NOTIFICATIONS,
    )
    .await;
    peripheral.unsubscribe(&config_char).await.unwrap();

    assert!(matching, "Should receive notification with custom payload [0xCA, 0xFE, 0xBA, 0xBE]");

    peripheral.disconnect().await.unwrap();
}
