mod common;

use btleplug::api::Peripheral as _;
use futures::StreamExt;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_subscribe_and_receive_indications() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::INDICATE_CHAR,
    );

    let mut stream = peripheral.notifications().await.unwrap();
    peripheral.subscribe(&char).await.unwrap();

    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_START_NOTIFICATIONS,
    )
    .await;

    let mut received = Vec::new();
    let timeout = time::sleep(Duration::from_secs(5));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(notification) = stream.next() => {
                if notification.uuid == common::gatt_uuids::INDICATE_CHAR {
                    received.push(notification);
                    if received.len() >= 2 {
                        break;
                    }
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
    peripheral.unsubscribe(&char).await.unwrap();

    assert!(
        received.len() >= 2,
        "Expected at least 2 indications, got {}",
        received.len()
    );

    peripheral.disconnect().await.unwrap();
}
