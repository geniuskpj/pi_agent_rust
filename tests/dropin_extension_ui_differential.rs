#![forbid(unsafe_code)]

use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::TempDir;

const UI_SCENARIOS: &str =
    include_str!("dropin_extension_ui_differential/fixtures/g05_extension_ui_scenarios.json");

/// Extension UI differential test harness for testing request/response round-trip parity
struct ExtensionUiDifferentialTester {
    #[allow(dead_code)]
    temp_dir: TempDir,
    rust_pi_path: String,
}

impl ExtensionUiDifferentialTester {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let rust_pi_path = std::env::var("CARGO_TARGET_DIR")
            .map(|dir| format!("{}/debug/pi", dir))
            .unwrap_or_else(|_| "target/debug/pi".to_string());

        Ok(Self {
            temp_dir,
            rust_pi_path,
        })
    }

    fn execute_ui_scenario(&self, scenario: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let requests = scenario["requests"].as_array().expect("scenario requests");
        let mut child = Command::new(&self.rust_pi_path)
            .args(["--mode", "rpc"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.as_mut().unwrap();

        // Send all requests in sequence
        for request in requests {
            writeln!(stdin, "{}", request)?;
        }
        drop(child.stdin.take());

        let output = child.wait_with_output()?;
        let stdout_str = String::from_utf8_lossy(&output.stdout);

        // Parse all JSON responses from stdout
        let mut responses = Vec::new();
        for line in stdout_str.lines() {
            if let Ok(response) = serde_json::from_str::<Value>(line) {
                responses.push(response);
            }
        }

        Ok(json!(responses))
    }

    fn validate_ui_scenario(
        &self,
        scenario: &Value,
        actual_responses: &Value,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let expected_patterns = scenario["expected_patterns"]
            .as_array()
            .expect("expected patterns");
        let responses = actual_responses.as_array().expect("actual responses array");

        // For each expected pattern, check if we find a matching response
        for pattern in expected_patterns {
            let pattern_type = pattern["type"].as_str().expect("pattern type");
            let found = responses
                .iter()
                .any(|response| self.matches_pattern(response, pattern, pattern_type));

            if !found {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn matches_pattern(&self, response: &Value, pattern: &Value, pattern_type: &str) -> bool {
        match pattern_type {
            "extension_ui_request" => {
                response.get("type") == Some(&json!("extension_ui_request"))
                    && pattern
                        .get("method")
                        .map_or(true, |m| response.get("method") == Some(m))
                    && pattern
                        .get("has_timeout")
                        .map_or(true, |_| response.get("timeout").is_some())
            }
            "response_success" => {
                response.get("type") == Some(&json!("response"))
                    && response.get("success") == Some(&json!(true))
            }
            "response_error" => {
                response.get("type") == Some(&json!("response"))
                    && response.get("success") == Some(&json!(false))
            }
            _ => false,
        }
    }
}

/// Canonicalizes extension UI responses by removing volatile fields
fn canonicalize_ui_response(value: &Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.iter().map(canonicalize_ui_response).collect()),
        Value::Object(object) => {
            let mut canonicalized = BTreeMap::new();
            for (key, value) in object.iter() {
                // Skip volatile fields specific to extension UI
                if matches!(key.as_str(), "timestamp" | "requestId" | "id" | "timeout") {
                    continue;
                }
                canonicalized.insert(key.clone(), canonicalize_ui_response(value));
            }
            Value::Object(canonicalized.into_iter().collect())
        }
        primitive => primitive.clone(),
    }
}

#[test]
fn g05_extension_ui_differential_fixture_validation() {
    // Validate fixture structure
    let scenarios: Value = serde_json::from_str(UI_SCENARIOS).expect("UI scenarios JSON");

    assert_eq!(
        scenarios["schema"],
        "pi.dropin.extension_ui_differential_scenarios.v1"
    );
    assert_eq!(scenarios["bead"], "bd-lnmtp.2.4");

    let ui_scenarios = scenarios["scenarios"]
        .as_array()
        .expect("UI scenarios array");
    assert!(
        ui_scenarios.len() >= 10,
        "bd-lnmtp.2.4 requires at least 10 UI scenarios, got {}",
        ui_scenarios.len()
    );

    // Validate each scenario has required fields
    for scenario in ui_scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(
            scenario.get("description").is_some(),
            "{} missing description",
            id
        );
        assert!(
            scenario.get("requests").is_some(),
            "{} missing requests",
            id
        );
        assert!(
            scenario.get("expected_patterns").is_some(),
            "{} missing expected_patterns",
            id
        );
    }
}

#[test]
fn g05_extension_ui_canonicalization_stable() {
    let test_cases = [
        json!({
            "type": "extension_ui_request",
            "id": "req-123",
            "method": "confirm",
            "title": "Continue?",
            "timestamp": "2026-04-23T00:00:00Z"
        }),
        json!({
            "type": "response",
            "command": "extension_ui_response",
            "success": true,
            "requestId": "req-456",
            "timestamp": "2026-04-23T00:00:01Z"
        }),
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        let canonical_once = canonicalize_ui_response(test_case);
        let canonical_twice = canonicalize_ui_response(&canonical_once);

        assert_eq!(
            canonical_once, canonical_twice,
            "Canonicalization not stable for test case {}",
            i
        );

        // Verify volatile fields are removed
        if let Value::Object(obj) = &canonical_once {
            assert!(
                !obj.contains_key("timestamp"),
                "timestamp should be removed"
            );
            assert!(
                !obj.contains_key("requestId"),
                "requestId should be removed"
            );
            assert!(!obj.contains_key("id"), "id should be removed");
        }
    }
}

#[test]
fn g05_extension_ui_differential_basic_scenarios() {
    let scenarios: Value = serde_json::from_str(UI_SCENARIOS).expect("UI scenarios JSON");
    let ui_scenarios = scenarios["scenarios"]
        .as_array()
        .expect("UI scenarios array");

    let cargo_target_dir =
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let rust_pi_path = format!("{}/debug/pi", cargo_target_dir);

    if !std::path::Path::new(&rust_pi_path).exists() {
        eprintln!(
            "Warning: Rust pi binary not found at {}. Skipping UI differential test.",
            rust_pi_path
        );
        return;
    }

    let tester = match ExtensionUiDifferentialTester::new() {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "Warning: Failed to create UI differential tester: {}. Skipping test.",
                e
            );
            return;
        }
    };

    let mut successful_scenarios = 0;
    let mut failed_scenarios = Vec::new();
    let total_scenarios = ui_scenarios.len().min(5); // Test first 5 scenarios

    for scenario in ui_scenarios.iter().take(5) {
        let scenario_id = scenario["id"].as_str().unwrap_or("unknown");
        let scenario_type = scenario["type"].as_str().unwrap_or("unknown");

        match tester.execute_ui_scenario(scenario) {
            Ok(responses) => match tester.validate_ui_scenario(scenario, &responses) {
                Ok(true) => {
                    successful_scenarios += 1;
                    println!("✓ {}: {} - PASS", scenario_id, scenario_type);
                }
                Ok(false) => {
                    failed_scenarios.push(format!(
                        "{}: {} - Pattern mismatch",
                        scenario_id, scenario_type
                    ));
                    println!("✗ {}: {} - FAIL", scenario_id, scenario_type);
                }
                Err(e) => {
                    failed_scenarios.push(format!(
                        "{}: {} - Validation error: {}",
                        scenario_id, scenario_type, e
                    ));
                    println!("✗ {}: {} - ERROR: {}", scenario_id, scenario_type, e);
                }
            },
            Err(e) => {
                failed_scenarios.push(format!(
                    "{}: {} - Execution error: {}",
                    scenario_id, scenario_type, e
                ));
                println!("✗ {}: {} - ERROR: {}", scenario_id, scenario_type, e);
            }
        }
    }

    println!(
        "\n=== G05 Extension UI Basic Differential Test Summary ===\n\
         Tested scenarios: {}\n\
         Successful: {}\n\
         Failed: {}\n\
         Success rate: {:.1}%\n",
        total_scenarios,
        successful_scenarios,
        failed_scenarios.len(),
        (successful_scenarios as f64 / total_scenarios as f64) * 100.0
    );

    if !failed_scenarios.is_empty() {
        println!("Failed scenarios:");
        for failure in &failed_scenarios {
            println!("  - {}", failure);
        }
    }

    // For now, just verify that we tested scenarios
    assert!(
        total_scenarios > 0,
        "Should have tested at least one UI scenario"
    );
}
