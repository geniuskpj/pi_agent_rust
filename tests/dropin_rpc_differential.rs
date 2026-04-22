#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

const SURFACE_DIFF: &str = include_str!("../docs/dropin-rpc-surface-diff.json");
const SCENARIOS: &str =
    include_str!("dropin_rpc_differential/fixtures/g05_rpc_surface_scenarios.json");

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Array(items) => {
            let mut canonical_items: Vec<_> = items.iter().map(canonicalize).collect();
            if canonical_items.iter().all(|item| {
                item.get("type")
                    .and_then(Value::as_str)
                    .is_some_and(|event_type| event_type.starts_with("tool_execution"))
            }) {
                canonical_items.sort_by(|left, right| {
                    let left_id = left.get("toolCallId").and_then(Value::as_str).unwrap_or("");
                    let right_id = right
                        .get("toolCallId")
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    left_id.cmp(right_id)
                });
            }
            Value::Array(canonical_items)
        }
        Value::Object(object) => Value::Object(
            object
                .iter()
                .filter(|(key, _)| !is_volatile_field(key))
                .map(|(key, value)| (key.clone(), canonicalize(value)))
                .collect(),
        ),
        primitive => primitive.clone(),
    }
}

fn is_volatile_field(key: &str) -> bool {
    matches!(
        key,
        "timestamp" | "durationMs" | "sessionFile" | "pid" | "sessionId" | "id" | "requestId"
    )
}

fn required_fields(entry: &Value) -> BTreeSet<String> {
    entry["schema"]["required"]
        .as_array()
        .or_else(|| entry["request"]["required"].as_array())
        .into_iter()
        .flatten()
        .map(|value| value.as_str().expect("required field string").to_string())
        .collect()
}

fn index_by_name<'a>(items: &'a [Value], kind: &str) -> BTreeMap<&'a str, &'a Value> {
    items
        .iter()
        .map(|item| {
            (
                item["name"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{kind} entry missing name: {item}")),
                item,
            )
        })
        .collect()
}

fn summary_count(surface: &Value, key: &str) -> usize {
    usize::try_from(
        surface["summary"][key]
            .as_u64()
            .unwrap_or_else(|| panic!("missing numeric summary key {key}")),
    )
    .unwrap_or_else(|_| panic!("summary key {key} does not fit usize"))
}

fn is_missing_field_negative_case(expected: &Value) -> bool {
    expected["success"] == false
        && expected["error"]
            .as_str()
            .is_some_and(|error| error.starts_with("Missing "))
}

fn assert_canonicalization_stable(id: &str, value: &Value) {
    let canonical_once = canonicalize(value);
    let canonical_twice = canonicalize(&canonical_once);
    assert_eq!(
        canonical_once, canonical_twice,
        "{id} canonicalization must be stable"
    );
}

fn assert_command_scenario(
    scenario: &Value,
    command_index: &BTreeMap<&str, &Value>,
    command_names: &mut BTreeSet<String>,
) {
    let id = scenario["id"].as_str().expect("command scenario id");
    let command = scenario["command"]
        .as_str()
        .expect("command scenario command");
    assert!(
        command_names.insert(command.to_string()),
        "duplicate command scenario for {command}"
    );

    let surface_entry = command_index
        .get(command)
        .unwrap_or_else(|| panic!("{id} references unknown command {command}"));
    assert_eq!(
        surface_entry["action"], "MATCH",
        "{id} must target a MATCH command"
    );
    assert_eq!(
        surface_entry["rust_status"], "implemented",
        "{id} must target implemented Rust command"
    );

    let input = scenario["input"].as_object().expect("command input object");
    assert_eq!(
        input.get("type").and_then(Value::as_str),
        Some(command),
        "{id} input type must match command"
    );

    let expected = &scenario["expect"];
    if !is_missing_field_negative_case(expected) {
        for required in required_fields(surface_entry) {
            assert!(
                input.contains_key(&required),
                "{id} missing required request field {required}"
            );
        }
    }

    assert_canonicalization_stable(id, expected);
    assert_eq!(
        expected["type"], "response",
        "{id} expected response envelope"
    );
    assert_eq!(expected["command"], command, "{id} expected command echo");
}

fn assert_event_scenario(
    scenario: &Value,
    event_index: &BTreeMap<&str, &Value>,
    event_names: &mut BTreeSet<String>,
) {
    let id = scenario["id"].as_str().expect("event scenario id");
    let event = scenario["event"].as_str().expect("event scenario event");
    assert!(
        event_names.insert(event.to_string()),
        "duplicate event scenario for {event}"
    );

    let surface_entry = event_index
        .get(event)
        .unwrap_or_else(|| panic!("{id} references unknown event {event}"));
    assert_eq!(
        surface_entry["action"], "MATCH",
        "{id} must target a MATCH event"
    );
    assert_eq!(
        surface_entry["rust_status"], "implemented",
        "{id} must target implemented Rust event"
    );

    let sample = &scenario["sample"];
    assert_eq!(sample["type"], event, "{id} sample type must match event");
    for required in required_fields(surface_entry) {
        assert!(
            sample.get(&required).is_some(),
            "{id} missing required event field {required}"
        );
    }
    assert_canonicalization_stable(id, sample);
}

#[test]
fn g05_rpc_differential_fixture_covers_matched_surface() {
    let surface: Value = serde_json::from_str(SURFACE_DIFF).expect("surface diff JSON");
    let scenarios: Value = serde_json::from_str(SCENARIOS).expect("scenario fixture JSON");

    assert_eq!(surface["schema"], "pi.dropin.rpc_surface_diff.v1");
    assert_eq!(
        scenarios["schema"],
        "pi.dropin.rpc_differential_scenarios.v1"
    );
    assert_eq!(scenarios["bead"], "bd-lnmtp.2.3");

    let commands = surface["commands"].as_array().expect("surface commands");
    let events = surface["events"].as_array().expect("surface events");
    let command_index = index_by_name(commands, "command");
    let event_index = index_by_name(events, "event");

    let command_scenarios = scenarios["commands"].as_array().expect("command scenarios");
    let event_scenarios = scenarios["events"].as_array().expect("event scenarios");
    let scenario_count = command_scenarios.len() + event_scenarios.len();
    assert!(
        scenario_count >= 25,
        "bd-lnmtp.2.3 requires at least 25 differential scenarios, got {scenario_count}"
    );
    assert_eq!(
        command_scenarios.len(),
        summary_count(&surface, "baseline_command_count"),
        "fixture must cover every baseline RPC command"
    );
    assert_eq!(
        event_scenarios.len(),
        summary_count(&surface, "baseline_event_count"),
        "fixture must cover every baseline RPC event"
    );

    let mut command_names = BTreeSet::new();
    for scenario in command_scenarios {
        assert_command_scenario(scenario, &command_index, &mut command_names);
    }

    let mut event_names = BTreeSet::new();
    for scenario in event_scenarios {
        assert_event_scenario(scenario, &event_index, &mut event_names);
    }

    assert!(
        surface["divergences"]
            .as_array()
            .expect("surface divergences")
            .is_empty(),
        "G05 differential harness expects no IMPLEMENT divergences"
    );
}

#[test]
fn g05_rpc_differential_canonicalization_stable() {
    let test_cases = [
        serde_json::json!({
            "type": "response",
            "command": "get_state",
            "success": true,
            "timestamp": "2026-04-22T00:00:00Z",
            "sessionId": "session-123",
            "data": {
                "nested_timestamp": "2026-04-22T00:00:01Z",
                "value": 42
            }
        }),
        serde_json::json!({
            "type": "tool_execution_start",
            "toolCallId": "tool-2",
            "timestamp": "2026-04-22T00:00:00Z",
            "sessionId": "session-456"
        }),
    ];

    for (index, test_case) in test_cases.iter().enumerate() {
        let id = format!("canonicalization-case-{index}");
        assert_canonicalization_stable(&id, test_case);
        let canonical = canonicalize(test_case);
        if let Value::Object(object) = canonical {
            assert!(!object.contains_key("timestamp"));
            assert!(!object.contains_key("sessionId"));
        }
    }
}

#[test]
fn g05_rpc_differential_tool_execution_sorting() {
    let unsorted = serde_json::json!([
        { "type": "tool_execution_start", "toolCallId": "tool-2", "toolName": "read" },
        { "type": "tool_execution_start", "toolCallId": "tool-1", "toolName": "write" },
        { "type": "tool_execution_end", "toolCallId": "tool-2", "result": "ok" }
    ]);

    let canonical = canonicalize(&unsorted);
    let Value::Array(items) = canonical else {
        panic!("Expected array after canonicalization");
    };
    assert_eq!(items[0]["toolCallId"], "tool-1");
    assert_eq!(items[1]["toolCallId"], "tool-2");
    assert_eq!(items[2]["toolCallId"], "tool-2");
}
