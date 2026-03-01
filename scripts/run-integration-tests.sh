#!/usr/bin/env bash
#
# Run each integration test individually to avoid multiple simultaneous
# BLE connections to the same test peripheral.
#
# Usage:
#   ./scripts/run-integration-tests.sh              # run all tests
#   ./scripts/run-integration-tests.sh test_discovery # run only tests from one module
#
# Environment:
#   BTLEPLUG_TEST_PERIPHERAL  - peripheral name (default: btleplug-test)
#   RUST_LOG                  - log level (e.g. debug, btleplug=trace)
#   DELAY                     - seconds to wait between tests (default: 2)

set -euo pipefail

DELAY="${DELAY:-2}"
TIMEOUT="${TIMEOUT:-20}"
PASSED=0
FAILED=0
SKIPPED=0
FAILURES=()

# Modules and their tests as parallel arrays (avoids bash 4+ associative arrays
# so the script works with macOS default bash 3.2).
MODULE_NAMES=(
  test_discovery
  test_connection
  test_read_write
  test_notifications
  test_descriptors
  test_device_info
)

MODULE_TESTS_test_discovery="
  test_discover_peripheral_by_name
  test_discover_services
  test_discover_characteristics
  test_scan_filter_by_service_uuid
  test_advertisement_manufacturer_data
  test_advertisement_services
"
MODULE_TESTS_test_connection="
  test_connect_and_disconnect
  test_reconnect_after_disconnect
  test_peripheral_triggered_disconnect
"
MODULE_TESTS_test_read_write="
  test_read_static_value
  test_read_counter_increments
  test_write_with_response
  test_write_without_response
  test_read_write_roundtrip
  test_long_value_read_write
  test_characteristic_properties
"
MODULE_TESTS_test_notifications="
  test_subscribe_and_receive_notifications
  test_subscribe_and_receive_indications
  test_unsubscribe_stops_notifications
  test_configurable_notification_payload
"
MODULE_TESTS_test_descriptors="
  test_read_only_descriptor
  test_read_write_descriptor_roundtrip
  test_descriptor_discovery
"
MODULE_TESTS_test_device_info="
  test_mtu_after_connection
  test_read_rssi
  test_properties_contain_peripheral_info
  test_connection_parameters
  test_request_connection_parameters
"

# Helper: get test list for a module via indirect variable expansion.
get_tests() {
  local varname="MODULE_TESTS_$1"
  echo "${!varname}"
}

# Helper: check if a module name is valid.
is_valid_module() {
  local name="$1"
  for mod in "${MODULE_NAMES[@]}"; do
    if [[ "$mod" == "$name" ]]; then
      return 0
    fi
  done
  return 1
}

# Filter to a single module if an argument was provided.
if [[ $# -gt 0 ]]; then
  filter="$1"
  if ! is_valid_module "$filter"; then
    echo "Unknown module: $filter"
    echo "Available modules: ${MODULE_NAMES[*]}"
    exit 1
  fi
  MODULE_NAMES=("$filter")
fi

total=0
for mod in "${MODULE_NAMES[@]}"; do
  for test_name in $(get_tests "$mod"); do
    total=$((total + 1))
  done
done

echo "=== btleplug integration tests ==="
echo "Running $total tests sequentially (${DELAY}s delay, ${TIMEOUT}s timeout per test)"
echo ""

test_num=0
for mod in "${MODULE_NAMES[@]}"; do
  echo "--- Module: $mod ---"
  for test_name in $(get_tests "$mod"); do
    test_num=$((test_num + 1))
    printf "[%2d/%2d] %-50s " "$test_num" "$total" "${mod}::${test_name}"

    if timeout "${TIMEOUT}s" cargo test --test "$mod" "$test_name" -- --ignored --exact 2>/tmp/btleplug-test-output.log; then
      echo "PASS"
      PASSED=$((PASSED + 1))
    else
      exit_code=$?
      if [[ $exit_code -eq 124 ]]; then
        echo "TIMEOUT (${TIMEOUT}s)"
      else
        echo "FAIL"
      fi
      FAILED=$((FAILED + 1))
      FAILURES+=("${mod}::${test_name}")
      # Show output for failed tests.
      echo "  --- output ---"
      sed 's/^/  /' /tmp/btleplug-test-output.log | tail -20
      echo "  --- end ---"
    fi

    # Brief delay to let the BLE stack settle between tests.
    if [[ $test_num -lt $total ]]; then
      sleep "$DELAY"
    fi
  done
  echo ""
done

rm -f /tmp/btleplug-test-output.log

echo "=== Results ==="
echo "  Passed:  $PASSED"
echo "  Failed:  $FAILED"
echo "  Total:   $total"

if [[ ${#FAILURES[@]} -gt 0 ]]; then
  echo ""
  echo "Failed tests:"
  for f in "${FAILURES[@]}"; do
    echo "  - $f"
  done
  exit 1
fi

echo ""
echo "All tests passed."
