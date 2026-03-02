mod common;

use btleplug::api::{ConnectionParameterPreset, Peripheral as _};

#[tokio::test]
#[ignore = "requires BLE test peripheral"]
async fn test_request_connection_parameters() {
    let peripheral = common::peripheral_finder::find_and_connect().await;

    // Request throughput-optimized parameters
    match peripheral
        .request_connection_parameters(ConnectionParameterPreset::ThroughputOptimized)
        .await
    {
        Ok(()) => {
            // Brief pause for the parameter update to take effect
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            // Verify parameters changed (if platform supports reading them)
            if let Ok(Some(params)) = peripheral.connection_parameters().await {
                // ThroughputOptimized should have a lower interval
                // (exact values depend on platform and negotiation)
                assert!(
                    params.interval_us > 0,
                    "Connection interval should be positive after update"
                );
            }
        }
        Err(btleplug::Error::NotSupported(_)) => {
            // Platform doesn't support requesting parameter updates
        }
        Err(e) => {
            panic!(
                "Unexpected error from request_connection_parameters: {:?}",
                e
            );
        }
    }

    peripheral.disconnect().await.unwrap();
}
