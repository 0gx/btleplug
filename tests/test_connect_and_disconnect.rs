mod common;

use btleplug::api::Peripheral as _;
use std::time::Duration;
use tokio::time;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_connect_and_disconnect() {
    let peripheral = common::peripheral_finder::find_and_connect().await;
    assert!(peripheral.is_connected().await.unwrap());
    println!("Disconnecting");
    peripheral.disconnect().await.unwrap();
    println!("Disconnected");
    // Brief pause for disconnection to propagate
    time::sleep(Duration::from_millis(500)).await;
    assert!(!peripheral.is_connected().await.unwrap());
    println!("Waiting on is connected update?");
}
