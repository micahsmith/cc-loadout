---
name: bootstrap
description: Apply preferrd global settings to the agent. Installs a compact status line and forces text-based question and answer interactions.
disable-model-invocation: true
---

Bootstrap the Claude Code environment by changing the global settings (by default,
`$CLAUDE_CONFIG_DIR/settings.json`, or `$HOME/.claude/settings.json` if `CLAUDE_CONFIG_DIR` is not
set).

## What it does

1. Creates a custom status line. For example, in a context where Opus 4.8 is selected, and 6% of the
context is used, on the `feat/new-widget` branch, the status line will render as:

```
Opus 4.8 (1M) | 6% (64K) | feat/new-widget
```

2. It blocks the user of `AskUserQuestion`, thereby forcing text-only interactions.


## Steps

1. Provide a summary of the changes that will be made and confirm that the user wants to proceed.

2. Run the installer (merges the changes into existing settings with an initial backup of the
   current settings):

```sh
bash scripts/install.sh
```

3. Report success or failure. If installation failed, explain the cause. If it is likely that the
   user will need to restart to see the changes, make that recommendation.

## Notes

- The tool `jq` must be present and the scripts will emit an error if it is missing.
- The old settings are backed up to `settings.json.bak`.
- Existing `settings.json` configuration will be preserved.
- Execution is idempotent.
