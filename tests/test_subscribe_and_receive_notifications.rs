mod common;

use btleplug::api::Peripheral as _;
use futures::StreamExt;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_subscribe_and_receive_notifications() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::NOTIFY_CHAR,
    );

    // Set up notification stream before subscribing
    let mut stream = peripheral.notifications().await.unwrap();

    // Subscribe to the notification characteristic
    peripheral.subscribe(&char).await.unwrap();

    // Tell the peripheral to start sending notifications
    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_START_NOTIFICATIONS,
    )
    .await;

    // Collect a few notifications (with timeout)
    let mut received = Vec::new();
    let timeout = time::sleep(Duration::from_secs(5));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(notification) = stream.next() => {
                if notification.uuid == common::gatt_uuids::NOTIFY_CHAR {
                    received.push(notification);
                    if received.len() >= 3 {
                        break;
                    }
                }
            }
            _ = &mut timeout => break,
        }
    }

    // Stop notifications and unsubscribe
    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_STOP_NOTIFICATIONS,
    )
    .await;
    peripheral.unsubscribe(&char).await.unwrap();

    assert!(
        received.len() >= 3,
        "Expected at least 3 notifications, got {}",
        received.len()
    );

    // Verify notifications have the correct service UUID
    for notif in &received {
        assert_eq!(notif.service_uuid, common::gatt_uuids::NOTIFICATION_SERVICE);
    }

    peripheral.disconnect().await.unwrap();
}
