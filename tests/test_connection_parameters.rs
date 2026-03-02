mod common;

use btleplug::api::Peripheral as _;

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_connection_parameters() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    match peripheral.connection_parameters().await {
        Ok(Some(params)) => {
            // Connection interval should be reasonable (7.5ms to 4000ms)
            assert!(
                params.interval_us >= 7_500 && params.interval_us <= 4_000_000,
                "Connection interval out of range: {} us",
                params.interval_us
            );
            // Latency should be 0-499
            assert!(
                params.latency <= 499,
                "Latency out of range: {}",
                params.latency
            );
            // Supervision timeout should be 100ms to 32s
            assert!(
                params.supervision_timeout_us >= 100_000
                    && params.supervision_timeout_us <= 32_000_000,
                "Supervision timeout out of range: {} us",
                params.supervision_timeout_us
            );
        }
        Ok(None) => {
            // Platform doesn't support reading connection parameters
        }
        Err(btleplug::Error::NotSupported(_)) => {
            // Platform doesn't implement this
        }
        Err(e) => {
            panic!("Unexpected error from connection_parameters: {:?}", e);
        }
    }

    peripheral.disconnect().await.unwrap();
}
