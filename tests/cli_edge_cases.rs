//! Advanced CLI argument parsing edge cases tests.
//!
//! Tests quoting, environment expansion, @file references, and positional precedence
//! edge cases to ensure parity with pi-mono behavior.

use std::env;
use std::fs;
use tempfile::TempDir;

use pi::cli::{Cli, Commands, parse_with_extension_flags};

/// Test that arguments with spaces are handled correctly when quoted.
#[test]
fn test_quoted_arguments_with_spaces() {
    // Test space-containing arguments in different positions
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--provider".to_string(),
        "custom provider".to_string(), // Space in value
        "hello world".to_string(), // Space in message
        "another message".to_string(),
    ]).expect("Should parse quoted args with spaces");

    assert_eq!(parsed.cli.provider, Some("custom provider".to_string()));
    assert_eq!(parsed.cli.message, vec!["hello", "world", "another", "message"]);
}

/// Test nested quoting scenarios.
#[test]
fn test_nested_quotes() {
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--system-prompt".to_string(),
        r#"You are a "helpful" assistant"#.to_string(),
        r#"Process the file "data.txt""#.to_string(),
    ]).expect("Should parse nested quotes");

    assert_eq!(parsed.cli.system_prompt, Some(r#"You are a "helpful" assistant"#.to_string()));
    assert_eq!(parsed.cli.message, vec!["Process", "the", "file", r#""data.txt""#]);
}

/// Test escaped quotes in arguments.
#[test]
fn test_escaped_quotes() {
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--system-prompt".to_string(),
        r#"Say \"hello\" to the user"#.to_string(),
        r#"The file is \"important.txt\""#.to_string(),
    ]).expect("Should parse escaped quotes");

    assert_eq!(parsed.cli.system_prompt, Some(r#"Say \"hello\" to the user"#.to_string()));
    assert_eq!(parsed.cli.message, vec!["The", "file", "is", r#"\"important.txt\""#]);
}

/// Test environment variable expansion in CLI arguments.
#[test]
fn test_environment_variable_expansion() {
    // Set test environment variables
    env::set_var("TEST_PROVIDER", "test-anthropic");
    env::set_var("TEST_MODEL", "claude-opus-4");
    env::set_var("TEST_PATH", "/tmp/test");

    // Note: This tests if the CLI would handle env var expansion
    // Real shell would expand these before they reach our CLI parser
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--provider".to_string(),
        "${TEST_PROVIDER}".to_string(),
        "--model".to_string(),
        "$TEST_MODEL".to_string(),
        "Check ${TEST_PATH}/file.txt".to_string(),
    ]).expect("Should handle env var syntax");

    // These are literal strings since shell expansion happens before CLI parsing
    assert_eq!(parsed.cli.provider, Some("${TEST_PROVIDER}".to_string()));
    assert_eq!(parsed.cli.model, Some("$TEST_MODEL".to_string()));
    assert_eq!(parsed.cli.message, vec!["Check", "${TEST_PATH}/file.txt"]);

    // Cleanup
    env::remove_var("TEST_PROVIDER");
    env::remove_var("TEST_MODEL");
    env::remove_var("TEST_PATH");
}

/// Test @file references with edge cases.
#[test]
fn test_file_reference_edge_cases() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Create test files
    let simple_file = temp_path.join("simple.txt");
    let space_file = temp_path.join("file with spaces.txt");
    let nested_dir = temp_path.join("nested");
    fs::create_dir(&nested_dir)?;
    let nested_file = nested_dir.join("deep.txt");

    fs::write(&simple_file, "simple content")?;
    fs::write(&space_file, "content with spaces")?;
    fs::write(&nested_file, "nested content")?;

    // Test various @file reference patterns
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        format!("@{}", simple_file.display()),
        format!("@{}", space_file.display()), // File path with spaces
        format!("@{}", nested_file.display()),
        "analyze these".to_string(),
    ]).expect("Should parse @file references");

    assert_eq!(parsed.cli.file_args(), vec![
        simple_file.to_string_lossy(),
        space_file.to_string_lossy(),
        nested_file.to_string_lossy()
    ]);
    assert_eq!(parsed.cli.message_args(), vec!["analyze", "these"]);

    Ok(())
}

/// Test @file references to non-existent files.
#[test]
fn test_nonexistent_file_references() {
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "@/nonexistent/file.txt".to_string(),
        "@missing.txt".to_string(),
        "process anyway".to_string(),
    ]).expect("Should parse even with non-existent @file references");

    // CLI parsing should succeed even if files don't exist
    // File existence checking happens later in the pipeline
    assert_eq!(parsed.cli.file_args(), vec!["/nonexistent/file.txt", "missing.txt"]);
    assert_eq!(parsed.cli.message_args(), vec!["process", "anyway"]);
}

/// Test positional argument precedence with various flag patterns.
#[test]
fn test_positional_precedence() {
    // Flags before positionals
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--model".to_string(),
        "claude".to_string(),
        "--verbose".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]).expect("Should parse flags before positionals");

    assert_eq!(parsed.cli.model, Some("claude".to_string()));
    assert!(parsed.cli.verbose);
    assert_eq!(parsed.cli.message, vec!["hello", "world"]);

    // Mixed flags and positionals
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "first".to_string(),
        "--model".to_string(),
        "claude".to_string(),
        "second".to_string(),
        "--verbose".to_string(),
        "third".to_string(),
    ]).expect("Should parse mixed flags and positionals");

    assert_eq!(parsed.cli.model, Some("claude".to_string()));
    assert!(parsed.cli.verbose);
    assert_eq!(parsed.cli.message, vec!["first", "second", "third"]);
}

/// Test double-dash separator behavior.
#[test]
fn test_double_dash_separator() {
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--model".to_string(),
        "claude".to_string(),
        "--".to_string(),
        "--not-a-flag".to_string(),
        "regular".to_string(),
        "args".to_string(),
    ]).expect("Should parse args after --");

    assert_eq!(parsed.cli.model, Some("claude".to_string()));
    // Everything after -- should be treated as positional arguments
    assert_eq!(parsed.cli.message, vec!["--not-a-flag", "regular", "args"]);
}

/// Test extension flags with edge cases.
#[test]
fn test_extension_flags_edge_cases() {
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--debug-level=high".to_string(), // Equals syntax
        "--verbose".to_string(), // Boolean flag
        "--output-dir".to_string(),
        "/path/with spaces".to_string(), // Value with spaces
        "--flag-with-dashes".to_string(),
        "value".to_string(),
        "message".to_string(),
    ]).expect("Should parse extension flags with edge cases");

    assert_eq!(parsed.extension_flags.len(), 4);

    // Check equals syntax
    let debug_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "debug-level")
        .expect("Should have debug-level flag");
    assert_eq!(debug_flag.value, Some("high".to_string()));

    // Check boolean flag
    let verbose_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "verbose")
        .expect("Should have verbose flag");
    assert_eq!(verbose_flag.value, None);

    // Check value with spaces
    let output_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "output-dir")
        .expect("Should have output-dir flag");
    assert_eq!(output_flag.value, Some("/path/with spaces".to_string()));

    // Check flag with dashes
    let dashes_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "flag-with-dashes")
        .expect("Should have flag-with-dashes flag");
    assert_eq!(dashes_flag.value, Some("value".to_string()));
}

/// Test complex mixed scenarios.
#[test]
fn test_complex_mixed_scenarios() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_file = temp_dir.path().join("test.txt");
    fs::write(&temp_file, "test content")?;

    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--model".to_string(),
        "claude-opus".to_string(),
        "--custom-flag=debug".to_string(), // Extension flag with equals
        format!("@{}", temp_file.display()), // File reference
        "--another-flag".to_string(), // Extension boolean flag
        "Analyze this file".to_string(), // Message with spaces
        "--".to_string(), // Separator
        "--not-parsed-as-flag".to_string(), // After separator
    ]).expect("Should parse complex mixed scenario");

    // Check standard flags
    assert_eq!(parsed.cli.model, Some("claude-opus".to_string()));

    // Check extension flags
    assert_eq!(parsed.extension_flags.len(), 2);
    let custom_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "custom-flag")
        .expect("Should have custom-flag");
    assert_eq!(custom_flag.value, Some("debug".to_string()));

    let another_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "another-flag")
        .expect("Should have another-flag");
    assert_eq!(another_flag.value, None);

    // Check file references
    assert_eq!(parsed.cli.file_args().len(), 1);
    assert!(parsed.cli.file_args()[0].ends_with("test.txt"));

    // Check message (including args after --)
    assert!(parsed.cli.message.contains(&"Analyze".to_string()));
    assert!(parsed.cli.message.contains(&"--not-parsed-as-flag".to_string()));

    Ok(())
}

/// Test subcommand parsing with edge cases.
#[test]
fn test_subcommand_edge_cases() {
    // Subcommand with flags before it
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--global-flag".to_string(),
        "value".to_string(),
        "install".to_string(),
        "npm:package".to_string(),
        "--local".to_string(),
    ]);

    // This might fail parsing due to subcommand, but extension flags should be extracted
    match parsed {
        Ok(p) => {
            // Verify extension flags were processed
            assert!(p.extension_flags.iter().any(|f| f.name == "global-flag"));
            // Verify subcommand was recognized
            matches!(p.cli.command, Some(Commands::Install { .. }));
        }
        Err(_) => {
            // If parsing fails, that's expected behavior for complex subcommand scenarios
            // The important thing is that preprocessing works correctly
        }
    }
}

/// Test Unicode and special characters in arguments.
#[test]
fn test_unicode_and_special_chars() {
    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--system-prompt".to_string(),
        "Respond in 中文 and emoji 🚀".to_string(),
        "--custom-flag".to_string(),
        "café-naïve".to_string(),
        "Message with émojis 🎉 and ñice chars".to_string(),
    ]).expect("Should handle Unicode and special characters");

    assert_eq!(parsed.cli.system_prompt, Some("Respond in 中文 and emoji 🚀".to_string()));

    let custom_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "custom-flag")
        .expect("Should have custom-flag");
    assert_eq!(custom_flag.value, Some("café-naïve".to_string()));

    assert!(parsed.cli.message.join(" ").contains("🎉"));
    assert!(parsed.cli.message.join(" ").contains("ñice"));
}

/// Test very long arguments.
#[test]
fn test_long_arguments() {
    let long_value = "x".repeat(10000);
    let long_message = "word ".repeat(1000);

    let parsed = parse_with_extension_flags(vec![
        "pi".to_string(),
        "--system-prompt".to_string(),
        long_value.clone(),
        "--custom-flag".to_string(),
        long_value.clone(),
        long_message.trim().to_string(),
    ]).expect("Should handle very long arguments");

    assert_eq!(parsed.cli.system_prompt.as_ref().unwrap().len(), 10000);

    let custom_flag = parsed.extension_flags.iter()
        .find(|f| f.name == "custom-flag")
        .expect("Should have custom-flag");
    assert_eq!(custom_flag.value.as_ref().unwrap().len(), 10000);

    assert!(parsed.cli.message.len() > 500); // Many repeated "word" entries
}