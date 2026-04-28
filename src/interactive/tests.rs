use super::*;
use crate::agent::AgentConfig;
use crate::model::StreamEvent;
use crate::provider::{Context, Provider, StreamOptions};
use crate::resources::{ResourceCliOptions, ResourceLoader};
use crate::tools::ToolRegistry;
use asupersync::channel::mpsc;
use asupersync::runtime::RuntimeBuilder;
use futures::stream;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, OnceLock};

struct DummyProvider;

#[async_trait::async_trait]
impl Provider for DummyProvider {
    fn name(&self) -> &'static str {
        "dummy"
    }

    fn api(&self) -> &'static str {
        "dummy"
    }

    fn model_id(&self) -> &'static str {
        "dummy-model"
    }

    async fn stream(
        &self,
        _context: &Context<'_>,
        _options: &StreamOptions,
    ) -> crate::error::Result<
        Pin<Box<dyn futures::Stream<Item = crate::error::Result<StreamEvent>> + Send>>,
    > {
        Ok(Box::pin(stream::empty()))
    }
}

fn test_runtime_handle() -> asupersync::runtime::RuntimeHandle {
    static RT: OnceLock<asupersync::runtime::Runtime> = OnceLock::new();
    RT.get_or_init(|| {
        RuntimeBuilder::current_thread()
            .build()
            .expect("build asupersync runtime")
    })
    .handle()
}

fn test_model_entry() -> ModelEntry {
    ModelEntry {
        model: crate::provider::Model {
            id: "gpt-5.2".to_string(),
            name: "gpt-5.2".to_string(),
            api: "openai-responses".to_string(),
            provider: "openai".to_string(),
            base_url: "https://example.invalid".to_string(),
            reasoning: true,
            input: vec![crate::provider::InputType::Text],
            cost: crate::provider::ModelCost {
                input: 0.0,
                output: 0.0,
                cache_read: 0.0,
                cache_write: 0.0,
            },
            context_window: 128_000,
            max_tokens: 8_192,
            headers: std::collections::HashMap::new(),
        },
        api_key: None,
        headers: std::collections::HashMap::new(),
        auth_header: false,
        compat: None,
        oauth_config: None,
    }
}

fn build_test_app(cwd: PathBuf) -> PiApp {
    let config = Config::default();
    let provider: Arc<dyn Provider> = Arc::new(DummyProvider);
    let agent = Agent::new(
        provider,
        ToolRegistry::new(&[], &cwd, Some(&config)),
        AgentConfig::default(),
    );
    let resources = ResourceLoader::empty(config.enable_skill_commands());
    let resource_cli = ResourceCliOptions {
        no_skills: false,
        no_prompt_templates: false,
        no_extensions: false,
        no_themes: false,
        skill_paths: Vec::new(),
        prompt_paths: Vec::new(),
        extension_paths: Vec::new(),
        theme_paths: Vec::new(),
    };
    let model_entry = test_model_entry();
    let (event_tx, _event_rx) = mpsc::channel(64);

    PiApp::new(
        agent,
        Arc::new(asupersync::sync::Mutex::new(Session::in_memory())),
        config,
        resources,
        resource_cli,
        cwd,
        model_entry.clone(),
        vec![model_entry.clone()],
        vec![model_entry],
        Vec::new(),
        event_tx,
        test_runtime_handle(),
        false,
        false,
        None,
        Some(KeyBindings::new()),
        Vec::new(),
        Usage::default(),
    )
}

fn tempdir() -> tempfile::TempDir {
    std::fs::create_dir_all(std::env::temp_dir()).expect("create temp root");
    tempfile::tempdir().expect("tempdir")
}

#[test]
fn prepare_startup_changelog_skips_disk_write_when_persistence_disabled() {
    let dir = tempdir();
    let cwd = dir.path().join("workspace");
    std::fs::create_dir_all(&cwd).expect("create cwd");
    let settings_path = dir.path().join("settings.json");
    let mut config = Config {
        last_changelog_version: Some("0.9.0".to_string()),
        ..Config::default()
    };

    let changelog = "## 1.0.0\n- Added startup changelog notices\n\n## 0.9.0\n- Previous release\n";
    let startup = prepare_startup_changelog_with_roots(
        &mut config,
        dir.path(),
        &cwd,
        Some(&settings_path),
        false,
        false,
        "1.0.0",
        changelog,
    );

    assert_eq!(
        startup,
        Some(StartupChangelog::Full {
            markdown: "## 1.0.0\n- Added startup changelog notices".to_string(),
        })
    );
    assert!(
        !settings_path.exists(),
        "startup construction should not write settings"
    );
    assert_eq!(config.last_changelog_version.as_deref(), Some("1.0.0"));
}

#[test]
fn prepare_startup_changelog_writes_when_persistence_enabled() {
    let dir = tempdir();
    let cwd = dir.path().join("workspace");
    std::fs::create_dir_all(&cwd).expect("create cwd");
    let settings_path = dir.path().join("settings.json");
    let mut config = Config {
        last_changelog_version: Some("0.9.0".to_string()),
        ..Config::default()
    };

    let startup = prepare_startup_changelog_with_roots(
        &mut config,
        dir.path(),
        &cwd,
        Some(&settings_path),
        false,
        true,
        "1.0.0",
        "## 1.0.0\n- Added startup changelog notices\n\n## 0.9.0\n- Previous release\n",
    );

    assert!(matches!(startup, Some(StartupChangelog::Full { .. })));
    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&settings_path).expect("read settings"))
            .expect("parse settings");
    assert_eq!(saved["lastChangelogVersion"].as_str(), Some("1.0.0"));
}

#[test]
fn extract_file_references_removes_indented_ref_line_without_leaving_blank_whitespace() {
    let dir = tempdir();
    std::fs::write(dir.path().join("notes.txt"), "hi").expect("write file");
    let mut app = build_test_app(dir.path().to_path_buf());

    let (cleaned, refs) = app.extract_file_references("Summary:\n  @notes.txt\nNext line");

    assert_eq!(cleaned, "Summary:\nNext line");
    assert_eq!(refs, vec!["notes.txt".to_string()]);
}

#[test]
fn extract_file_references_preserves_newline_before_trailing_punctuation() {
    let dir = tempdir();
    std::fs::write(dir.path().join("notes.txt"), "hi").expect("write file");
    let mut app = build_test_app(dir.path().to_path_buf());

    let (cleaned, refs) = app.extract_file_references("Summary:\n@notes.txt.");

    assert_eq!(cleaned, "Summary:\n.");
    assert_eq!(refs, vec!["notes.txt".to_string()]);
}

#[test]
fn is_inside_jj_repo_detects_root_directly() {
    let dir = tempdir();
    std::fs::create_dir(dir.path().join(".jj")).expect("mkdir .jj");
    assert!(super::is_inside_jj_repo(dir.path()));
}

#[test]
fn is_inside_jj_repo_walks_up_to_ancestor() {
    let dir = tempdir();
    let root = dir.path();
    std::fs::create_dir(root.join(".jj")).expect("mkdir .jj");
    let nested = root.join("a").join("b").join("c");
    std::fs::create_dir_all(&nested).expect("mkdir nested");
    assert!(super::is_inside_jj_repo(&nested));
}

#[test]
fn is_inside_jj_repo_false_when_no_dot_jj_anywhere() {
    let dir = tempdir();
    let nested = dir.path().join("a").join("b");
    std::fs::create_dir_all(&nested).expect("mkdir nested");
    assert!(!super::is_inside_jj_repo(&nested));
}

#[test]
fn is_inside_jj_repo_requires_dot_jj_to_be_a_directory() {
    // A file named `.jj` is a gitlink-like stub in some tooling; only a
    // real `.jj/` directory counts as a jj repo for display purposes.
    let dir = tempdir();
    std::fs::write(dir.path().join(".jj"), "not a dir").expect("write stub");
    assert!(!super::is_inside_jj_repo(dir.path()));
}

#[test]
fn read_jj_change_returns_none_outside_jj_repo() {
    // No `.jj` anywhere -> must short-circuit without forking a
    // subprocess and without touching $PATH for the `jj` binary.
    let dir = tempdir();
    assert!(super::read_jj_change(dir.path()).is_none());
}

#[test]
fn read_vcs_info_falls_back_to_git_when_no_jj() {
    // Seed a minimal `.git/HEAD` pointing at a branch. With no `.jj`
    // anywhere, read_vcs_info must return the git branch name unchanged.
    let dir = tempdir();
    let dot_git = dir.path().join(".git");
    std::fs::create_dir(&dot_git).expect("mkdir .git");
    std::fs::write(dot_git.join("HEAD"), "ref: refs/heads/feature/jj-demo\n").expect("seed HEAD");

    let vcs = super::read_vcs_info(dir.path());
    assert_eq!(vcs.as_deref(), Some("feature/jj-demo"));
}

#[test]
fn render_header_uses_cycle_thinking_binding_hint() {
    let dir = tempdir();
    let mut app = build_test_app(dir.path().to_path_buf());
    app.set_terminal_size(200, 40);

    let header = app.render_header();

    assert!(header.contains("shift+tab: thinking"), "header: {header}");
    assert!(!header.contains("ctrl+t: thinking"), "header: {header}");
}

#[test]
fn enter_accepts_highlighted_autocomplete_item() {
    // Regression for issue #61: with the slash dropdown open and an entry
    // highlighted (e.g. user pressed Down to select `/model`), pressing Enter
    // must accept the highlighted item — matching the dropdown's own footer
    // hint "Enter/Tab accept" — not submit the raw `/` typed so far.
    use crate::autocomplete::{AutocompleteItem, AutocompleteItemKind};
    use bubbletea::{KeyMsg, KeyType, Message, Model as BubbleteaModel};

    let dir = tempdir();
    let mut app = build_test_app(dir.path().to_path_buf());

    app.input.set_value("/");
    app.autocomplete.open = true;
    app.autocomplete.items = vec![AutocompleteItem {
        kind: AutocompleteItemKind::SlashCommand,
        label: "/model".to_string(),
        insert: "/model ".to_string(),
        description: None,
    }];
    app.autocomplete.selected = Some(0);
    app.autocomplete.replace_range = 0..1;

    let _ = app.update(Message::new(KeyMsg::from_type(KeyType::Enter)));

    assert_eq!(
        app.input.value(),
        "/model ",
        "Enter with a highlighted dropdown entry must accept the item"
    );
    assert!(
        !app.autocomplete.open,
        "Accepting via Enter should close the dropdown"
    );
}

#[test]
fn enter_submits_when_no_autocomplete_item_highlighted() {
    // The dual contract for issue #61: when the dropdown is open but the
    // user has not navigated to any item (selected.is_none()), Enter must
    // still submit the raw editor contents — i.e. behavior is unchanged
    // for users who never pressed Down.
    use bubbletea::{KeyMsg, KeyType, Message, Model as BubbleteaModel};

    let dir = tempdir();
    let mut app = build_test_app(dir.path().to_path_buf());

    app.input.set_value("/foo");
    app.autocomplete.open = true;
    app.autocomplete.items.clear();
    app.autocomplete.selected = None;
    app.autocomplete.replace_range = 0..4;

    let _ = app.update(Message::new(KeyMsg::from_type(KeyType::Enter)));

    assert!(
        !app.autocomplete.open,
        "Enter with no selection should still close the dropdown"
    );
}
