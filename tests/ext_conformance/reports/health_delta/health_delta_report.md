# Extension Health & Regression Delta Report

> Generated: 2026-05-01T04:10:28Z
> Baseline: 2026-02-07T23:31:53Z

## Aggregate Comparison

| Metric | Baseline | Current | Delta |
|--------|----------|---------|-------|
| Tested | 223 | 224 | +1 |
| Passed | 187 | 221 | +34 |
| Failed | 36 | 3 | -33 |
| Pass rate | 83.9% | 98.7% | +14.8pp |

## Delta Summary

| Category | Count |
|----------|-------|
| Regressions | 0 |
| Fixes | 34 |
| New extensions | 188 |
| Removed | 0 |
| Unchanged failures | 2 |
| **Net change** | **+34** |

## Fixes (was failing, now passing)

| Extension | Tier | Previous Reason |
|-----------|------|-----------------|
| agents-mikeastock/extensions | 5 | Extension loads but registers different commands/tools than manifest expects. |
| base_fixtures | 3 | Not a real extension; test-only fixture in manifest. |
| community/nicobailon-interview-tool | 4 | Extension crashes during initialization (missing data, broken API, FS dependency). |
| community/prateekmedia-lsp | 3 | Extension loads but registers different commands/tools than manifest expects. |
| community/qualisero-background-notify | 3 | Extension uses relative imports to unbundled sibling/parent modules. |
| community/qualisero-pi-agent-scip | 3 | Extension uses relative imports to unbundled sibling/parent modules. |
| community/qualisero-safe-git | 3 | Extension uses relative imports to unbundled sibling/parent modules. |
| npm/aliou-pi-guardrails | 3 | Extension crashes during initialization (missing data, broken API, FS dependency). |
| npm/aliou-pi-linkup | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/aliou-pi-synthetic | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/aliou-pi-toolchain | 3 | Extension crashes during initialization (missing data, broken API, FS dependency). |
| npm/lsp-pi | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/marckrenn-pi-sub-bar | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/marckrenn-pi-sub-core | 3 | Extension crashes during initialization (missing data, broken API, FS dependency). |
| npm/mitsupi | 5 | Extension loads but registers different commands/tools than manifest expects. |
| npm/pi-amplike | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/pi-bash-confirm | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/pi-extensions | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/pi-package-test | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/pi-search-agent | 3 | Extension requires an npm package not available as a virtual module stub. |
| npm/pi-wakatime | 3 | Extension requires an npm package not available as a virtual module stub. |
| npm/pi-web-access | 3 | Extension requires an npm package not available as a virtual module stub. |
| npm/qualisero-pi-agent-scip | 3 | Extension requires an npm package not available as a virtual module stub. |
| npm/shitty-extensions | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/tmustier-pi-arcade | 3 | Extension loads but registers different commands/tools than manifest expects. |
| npm/vaayne-agent-kit | 3 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/ben-vargas-pi-packages | 3 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/charles-cooper-pi-extensions | 3 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/kcosr-pi-extensions | 5 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/marckrenn-pi-sub | 3 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/openclaw-openclaw | 3 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/pasky-pi-amplike | 3 | Extension loads but registers different commands/tools than manifest expects. |
| third-party/qualisero-pi-agent-scip | 3 | Extension requires an npm package not available as a virtual module stub. |
| third-party/w-winter-dot314 | 3 | Extension loads but registers different commands/tools than manifest expects. |

