//! Integration tests for slash command differential parity.
//!
//! This test suite verifies that slash commands in Rust Pi produce
//! equivalent behavior to pi-mono through automated RPC testing.

#[path = "dropin_slash_differential/mod.rs"]
mod dropin_slash_differential;
use dropin_slash_differential::*;

/// Comprehensive test of slash command parity across all supported commands.
#[test]
fn test_slash_command_differential_parity() {
    let tester = DifferentialTester::new().expect("Failed to create differential tester");

    let results = tester.run_all_scenarios();

    // Collect any failures
    let mut failures = Vec::new();
    let mut total_scenarios = 0;
    let mut successful_scenarios = 0;

    for (scenario_name, result) in &results {
        total_scenarios += 1;
        if result.success {
            successful_scenarios += 1;
        } else {
            failures.push(format!(
                "Scenario '{}' failed with differences: {:?}",
                scenario_name, result.differences
            ));
        }
    }

    // Print summary
    println!(
        "\n=== Slash Command Differential Test Summary ===\n\
         Total scenarios: {}\n\
         Successful: {}\n\
         Failed: {}\n",
        total_scenarios,
        successful_scenarios,
        total_scenarios - successful_scenarios
    );

    if !failures.is_empty() {
        println!("Failures:");
        for failure in &failures {
            println!("  - {failure}");
        }
    }

    // Test should pass for now since we're using placeholders
    // TODO: Update this when implementing actual differential testing
    assert_eq!(failures.len(), 0, "Differential test failures detected");
}

/// Test that basic slash command parsing works correctly.
#[test]
fn test_slash_command_parsing() {
    // Verify that our test scenarios cover the actual slash commands
    // supported by the Rust implementation
    let tester = DifferentialTester::new().expect("Failed to create tester");

    // Check that we have test scenarios for core commands
    let scenario_commands: Vec<String> =
        tester.scenarios.iter().map(|s| s.command.clone()).collect();

    // Verify coverage of essential commands
    let essential_commands = vec![
        "/help",
        "/h",
        "/?",
        "/clear",
        "/cls",
        "/model",
        "/m",
        "/thinking",
        "/t",
        "/exit",
        "/quit",
        "/q",
        "/session",
        "/info",
        "/tree",
        "/compact",
    ];

    for essential in essential_commands {
        assert!(
            scenario_commands
                .iter()
                .any(|cmd| cmd.starts_with(essential)),
            "Missing test scenario for essential command: {essential}"
        );
    }
}

/// Test response canonicalization functionality.
#[test]
fn test_response_canonicalization() {
    use serde_json::json;

    let test_response = json!({
        "status": "success",
        "timestamp": "2024-04-22T17:49:00Z",
        "id": "req-test-123",
        "duration": 150,
        "path": "/tmp/test-session",
        "data": {
            "message": "Command executed",
            "nested_timestamp": "2024-04-22T17:49:01Z",
            "tokens": 42
        }
    });

    let canonicalized = canonicalize_response(test_response);

    // Non-deterministic fields should be removed
    assert!(canonicalized.get("timestamp").is_none());
    assert!(canonicalized.get("id").is_none());
    assert!(canonicalized.get("duration").is_none());
    assert!(canonicalized["data"].get("nested_timestamp").is_none());

    // Deterministic fields should be preserved
    assert_eq!(canonicalized["status"], "success");
    assert_eq!(canonicalized["data"]["message"], "Command executed");
    assert_eq!(canonicalized["data"]["tokens"], 42);
}

/// Test combinatorial slash command scenarios.
#[test]
fn test_combinatorial_slash_commands() {
    let mut tester = DifferentialTester::new().expect("Failed to create tester");

    // Add combinatorial test scenarios
    tester.add_scenario(SlashCommandScenario {
        name: "model_then_thinking".to_string(),
        command: "/thinking high".to_string(),
        description: "Set thinking level after potential model change".to_string(),
        supports_streaming: false,
        setup: vec!["/model".to_string()], // First show model selector
    });

    tester.add_scenario(SlashCommandScenario {
        name: "clear_then_help".to_string(),
        command: "/help".to_string(),
        description: "Help command should work after clearing history".to_string(),
        supports_streaming: false,
        setup: vec!["some conversation".to_string(), "/clear".to_string()],
    });

    tester.add_scenario(SlashCommandScenario {
        name: "multiple_thinking_changes".to_string(),
        command: "/thinking off".to_string(),
        description: "Multiple thinking level changes should work".to_string(),
        supports_streaming: false,
        setup: vec!["/thinking high".to_string(), "/thinking medium".to_string()],
    });

    // Run just the combinatorial scenarios
    let combinatorial_scenarios: Vec<_> = tester
        .scenarios
        .iter()
        .filter(|s| {
            s.name.contains("model_then_")
                || s.name.contains("clear_then_")
                || s.name.contains("multiple_")
        })
        .cloned()
        .collect();

    for scenario in combinatorial_scenarios {
        let result = DifferentialTester::run_scenario(&scenario);

        // For now, expect success with placeholder implementation
        assert!(
            result.success,
            "Combinatorial scenario '{}' should succeed",
            scenario.name
        );
    }
}

/// Test error handling for invalid slash commands.
#[test]
fn test_invalid_slash_command_handling() {
    let mut tester = DifferentialTester::new().expect("Failed to create tester");

    // Add invalid command scenarios
    let invalid_scenarios = vec![
        SlashCommandScenario {
            name: "invalid_command".to_string(),
            command: "/nonexistent".to_string(),
            description: "Invalid slash command should be handled gracefully".to_string(),
            supports_streaming: false,
            setup: vec![],
        },
        SlashCommandScenario {
            name: "malformed_thinking".to_string(),
            command: "/thinking invalid_level".to_string(),
            description: "Invalid thinking level should show error".to_string(),
            supports_streaming: false,
            setup: vec![],
        },
        SlashCommandScenario {
            name: "empty_slash".to_string(),
            command: "/".to_string(),
            description: "Empty slash command should be handled".to_string(),
            supports_streaming: false,
            setup: vec![],
        },
    ];

    for scenario in invalid_scenarios {
        tester.add_scenario(scenario.clone());
        let result = DifferentialTester::run_scenario(&scenario);

        // Invalid commands should still complete (with appropriate error responses)
        // For placeholder implementation, expect success
        assert!(
            result.success,
            "Invalid command scenario '{}' should handle errors gracefully",
            scenario.name
        );
    }
}
