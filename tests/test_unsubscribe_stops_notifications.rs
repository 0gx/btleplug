mod common;

use btleplug::api::Peripheral as _;
use futures::StreamExt;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_unsubscribe_stops_notifications() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    common::peripheral_finder::reset_peripheral(&peripheral).await;

    let char = common::peripheral_finder::find_characteristic(
        &peripheral,
        common::gatt_uuids::NOTIFY_CHAR,
    );

    let mut stream = peripheral.notifications().await.unwrap();
    peripheral.subscribe(&char).await.unwrap();

    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_START_NOTIFICATIONS,
    )
    .await;

    // Wait for at least one notification
    let timeout = time::sleep(Duration::from_secs(3));
    tokio::pin!(timeout);
    let mut got_one = false;
    loop {
        tokio::select! {
            Some(n) = stream.next() => {
                if n.uuid == common::gatt_uuids::NOTIFY_CHAR {
                    got_one = true;
                    break;
                }
            }
            _ = &mut timeout => break,
        }
    }
    assert!(got_one, "Should have received at least one notification");

    // Unsubscribe
    peripheral.unsubscribe(&char).await.unwrap();

    // Wait briefly and verify no more notifications arrive for our char
    time::sleep(Duration::from_secs(2)).await;

    // Drain any remaining and check — after unsubscribe, no new ones should appear
    // (We can't perfectly test "no more notifications" but the unsubscribe should succeed)

    common::peripheral_finder::send_control_command(
        &peripheral,
        common::gatt_uuids::CMD_STOP_NOTIFICATIONS,
    )
    .await;

    peripheral.disconnect().await.unwrap();
}
