# End-User CLI Extension Journey Report

> Generated: 2026-05-01T04:07:47Z

## Summary

| Metric | Value |
|--------|-------|
| Must-pass total | 123 |
| Tested | 123 |
| Passed | 87 |
| Failed | 36 |
| Skipped | 0 |
| Pass rate | 70.7% |

## By Journey Category

| Category | Pass | Fail | Skip |
|----------|------|------|------|
| command_provider | 33 | 0 | 0 |
| event_subscriber | 0 | 35 | 0 |
| multi_capability | 41 | 1 | 0 |
| passive | 2 | 0 | 0 |
| tool_provider | 11 | 0 | 0 |

## Journey Failures

### auto-commit-on-exit (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_auto_commit_on_exit --nocapture --exact
  ```

### claude-rules (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_claude_rules --nocapture --exact
  ```

### community/ferologics-notify (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_ferologics_notify --nocapture --exact
  ```

### community/mitsuhiko-cwd-history (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_mitsuhiko_cwd_history --nocapture --exact
  ```

### community/mitsuhiko-notify (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_mitsuhiko_notify --nocapture --exact
  ```

### community/mitsuhiko-whimsical (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_mitsuhiko_whimsical --nocapture --exact
  ```

### community/nicobailon-rewind-hook (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_nicobailon_rewind_hook --nocapture --exact
  ```

### community/ogulcancelik-ghostty-theme-sync (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_ogulcancelik_ghostty_theme_sync --nocapture --exact
  ```

### community/prateekmedia-token-rate (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_prateekmedia_token_rate --nocapture --exact
  ```

### community/tmustier-agent-guidance (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_community_tmustier_agent_guidance --nocapture --exact
  ```

### confirm-destructive (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_confirm_destructive --nocapture --exact
  ```

### custom-compaction (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_custom_compaction --nocapture --exact
  ```

### dirty-repo-guard (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_dirty_repo_guard --nocapture --exact
  ```

### dynamic-resources (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_dynamic_resources --nocapture --exact
  ```

### file-trigger (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_file_trigger --nocapture --exact
  ```

### git-checkpoint (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_git_checkpoint --nocapture --exact
  ```

### inline-bash (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_inline_bash --nocapture --exact
  ```

### input-transform (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_input_transform --nocapture --exact
  ```

### interactive-shell (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_interactive_shell --nocapture --exact
  ```

### mac-system-theme (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_mac_system_theme --nocapture --exact
  ```

### modal-editor (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_modal_editor --nocapture --exact
  ```

### model-status (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_model_status --nocapture --exact
  ```

### notify (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_notify --nocapture --exact
  ```

### npm/pi-ghostty-theme-sync (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_npm_pi_ghostty_theme_sync --nocapture --exact
  ```

### npm/pi-notify (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_npm_pi_notify --nocapture --exact
  ```

### npm/pi-prompt-template-model (tier 2)

- **Category:** MultiCapability
- **Journey:** Load extension -> verify all registration types -> cross-check capabilities
- **Failed at:** verify_all_registrations
- **Reason:** Expected commands registration but none found
- **Progress:** 1/4 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_npm_pi_prompt_template_model --nocapture --exact
  ```

### npm/token-rate-pi (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_npm_token_rate_pi --nocapture --exact
  ```

### permission-gate (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_permission_gate --nocapture --exact
  ```

### prompt-url-widget (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_prompt_url_widget --nocapture --exact
  ```

### protected-paths (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_protected_paths --nocapture --exact
  ```

### rainbow-editor (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_rainbow_editor --nocapture --exact
  ```

### status-line (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_status_line --nocapture --exact
  ```

### system-prompt-header (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_system_prompt_header --nocapture --exact
  ```

### third-party/rytswd-direnv (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_third_party_rytswd_direnv --nocapture --exact
  ```

### titlebar-spinner (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_titlebar_spinner --nocapture --exact
  ```

### widget-placement (tier 2)

- **Category:** EventSubscriber
- **Journey:** Load extension -> verify event handler registration -> check subscriptions
- **Failed at:** verify_event_registration
- **Reason:** Manifest expects event subscriptions but none registered
- **Progress:** 1/3 steps
- **Reproduce:**
  ```bash
  cargo test --test ext_conformance_generated --features ext-conformance -- ext_widget_placement --nocapture --exact
  ```

