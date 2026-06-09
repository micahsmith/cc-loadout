---
name: bootstrap
description: Bootstrap the agent execution environment to preferred global settings. Installs a compact status line and disables interactive answer selection.
disable-model-invocation: true
---

Bootstrap the Claude Code environment by changing the global settings (by default,
`$CLAUDE_CONFIG_DIR`, otherwise `~/.claude/settings.json`).

## What it does

1. Creates a custom status line. For example, in a context where Opus 4.8 is selected, and 6% of the
context is used, on the `feat/new-widget` branch, the status line will render as:

```
Opus 4.8 (1M) | 6% (64K) | feat/new-widget
```

2. It automatically denies the user of `AskUserQuestion` so the user will only interact via text.


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
