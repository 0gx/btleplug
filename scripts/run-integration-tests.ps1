#
# Run each integration test individually to avoid multiple simultaneous
# BLE connections to the same test peripheral.
#
# Usage:
#   .\scripts\run-integration-tests.ps1                    # run all tests
#   .\scripts\run-integration-tests.ps1 test_discovery     # run only tests from one module
#
# Environment:
#   BTLEPLUG_TEST_PERIPHERAL  - peripheral name (default: btleplug-test)
#   RUST_LOG                  - log level (e.g. debug, btleplug=trace)
#   DELAY                     - seconds to wait between tests (default: 2)
#   TIMEOUT                   - seconds before a test is killed (default: 20)

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$Module
)

$ErrorActionPreference = 'Stop'

$Delay   = if ($env:DELAY)   { [int]$env:DELAY }   else { 2 }
$Timeout = if ($env:TIMEOUT) { [int]$env:TIMEOUT } else { 20 }
$Passed  = 0
$Failed  = 0
$Failures = @()

$ModuleTests = [ordered]@{
    test_discovery = @(
        'test_discover_peripheral_by_name'
        'test_discover_services'
        'test_discover_characteristics'
        'test_scan_filter_by_service_uuid'
        'test_advertisement_manufacturer_data'
        'test_advertisement_services'
    )
    test_connection = @(
        'test_connect_and_disconnect'
        'test_reconnect_after_disconnect'
        'test_peripheral_triggered_disconnect'
    )
    test_read_write = @(
        'test_read_static_value'
        'test_read_counter_increments'
        'test_write_with_response'
        'test_write_without_response'
        'test_read_write_roundtrip'
        'test_long_value_read_write'
        'test_characteristic_properties'
    )
    test_notifications = @(
        'test_subscribe_and_receive_notifications'
        'test_subscribe_and_receive_indications'
        'test_unsubscribe_stops_notifications'
        'test_configurable_notification_payload'
    )
    test_descriptors = @(
        'test_read_only_descriptor'
        'test_read_write_descriptor_roundtrip'
        'test_descriptor_discovery'
    )
    test_device_info = @(
        'test_mtu_after_connection'
        'test_read_rssi'
        'test_properties_contain_peripheral_info'
        'test_connection_parameters'
        'test_request_connection_parameters'
    )
}

# Filter to a single module if an argument was provided.
if ($Module) {
    if (-not $ModuleTests.Contains($Module)) {
        Write-Host "Unknown module: $Module"
        Write-Host "Available modules: $($ModuleTests.Keys -join ', ')"
        exit 1
    }
    $RunModules = @($Module)
} else {
    $RunModules = @($ModuleTests.Keys)
}

$Total = 0
foreach ($mod in $RunModules) {
    $Total += $ModuleTests[$mod].Count
}

Write-Host "=== btleplug integration tests ==="
Write-Host "Running $Total tests sequentially (${Delay}s delay, ${Timeout}s timeout per test)"
Write-Host ""

$LogFile = Join-Path $env:TEMP 'btleplug-test-output.log'
$TestNum = 0

foreach ($mod in $RunModules) {
    Write-Host "--- Module: $mod ---"
    foreach ($testName in $ModuleTests[$mod]) {
        $TestNum++
        $label = "${mod}::${testName}"
        Write-Host -NoNewline ("[{0,2}/{1,2}] {2,-50} " -f $TestNum, $Total, $label)

        # Run cargo test with a timeout via Start-Process.
        $proc = Start-Process -FilePath 'cargo' `
            -ArgumentList "test --test $mod $testName -- --ignored --exact" `
            -NoNewWindow -PassThru `
            -RedirectStandardError $LogFile `
            -RedirectStandardOutput "$LogFile.stdout"

        $finished = $proc.WaitForExit($Timeout * 1000)

        if (-not $finished) {
            # Timed out — kill the process tree.
            try { $proc.Kill($true) } catch {}
            $proc.WaitForExit()
            Write-Host "TIMEOUT (${Timeout}s)"
            $Failed++
            $Failures += $label
            Write-Host "  --- output ---"
            if (Test-Path $LogFile) {
                Get-Content $LogFile -Tail 20 | ForEach-Object { "  $_" }
            }
            if (Test-Path "$LogFile.stdout") {
                Get-Content "$LogFile.stdout" -Tail 20 | ForEach-Object { "  $_" }
            }
            Write-Host "  --- end ---"
        } elseif ($proc.ExitCode -ne 0) {
            Write-Host "FAIL"
            $Failed++
            $Failures += $label
            Write-Host "  --- output ---"
            if (Test-Path $LogFile) {
                Get-Content $LogFile -Tail 20 | ForEach-Object { "  $_" }
            }
            if (Test-Path "$LogFile.stdout") {
                Get-Content "$LogFile.stdout" -Tail 20 | ForEach-Object { "  $_" }
            }
            Write-Host "  --- end ---"
        } else {
            Write-Host "PASS"
            $Passed++
        }

        # Brief delay to let the BLE stack settle between tests.
        if ($TestNum -lt $Total) {
            Start-Sleep -Seconds $Delay
        }
    }
    Write-Host ""
}

Remove-Item -Path $LogFile, "$LogFile.stdout" -ErrorAction SilentlyContinue

Write-Host "=== Results ==="
Write-Host "  Passed:  $Passed"
Write-Host "  Failed:  $Failed"
Write-Host "  Total:   $Total"

if ($Failures.Count -gt 0) {
    Write-Host ""
    Write-Host "Failed tests:"
    foreach ($f in $Failures) {
        Write-Host "  - $f"
    }
    exit 1
}

Write-Host ""
Write-Host "All tests passed."
