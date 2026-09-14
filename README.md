# cc-loadout

Skills and hooks library for Claude Code and other agents.

## Installation

This repository is a Claude Code plugin marketplace named `cc-loadout`. The marketplace provides
one plugin, named `code-skillset`.

### Install

Run these commands in a terminal:

```sh
claude plugin marketplace add https://github.com/micahsmith/cc-loadout.git
claude plugin install code-skillset@cc-loadout
```

Inside a Claude Code session, use the `/plugin` equivalents instead:

```text
/plugin marketplace add https://github.com/micahsmith/cc-loadout.git
/plugin install code-skillset@cc-loadout
```

The commands use an HTTPS URL, so they work without SSH keys. Both commands are safe to re-run.
Restart Claude Code to load the plugin.

### Update

Updates track the latest commit on the default branch:

```sh
claude plugin marketplace update cc-loadout
claude plugin update code-skillset@cc-loadout
```

Restart Claude Code to apply the update.

### Reinstall

A reinstall removes all cached state and starts from a clean copy. Use it if an install or update
fails:

```sh
claude plugin uninstall code-skillset@cc-loadout
claude plugin marketplace remove cc-loadout
claude plugin marketplace add https://github.com/micahsmith/cc-loadout.git
claude plugin install code-skillset@cc-loadout
```

## Hooks

The hooks inject these guides at `SessionStart` and `SubagentStart`.

| Guide | Purpose |
|-------|---------|
| `how-to-write-comments` | Rules for writing code comments and doc comments. |
| `how-to-write-prose` | Rules for technical writing that improve clarity and comprehension. |

## Skills

| Skill | Purpose |
|-------|---------|
| `bootstrap` | Apply preferred global settings. |
| `brainstorm` | Interview the user to reach alignment on design and requirements before work begins. |
| `cleanup-comments` | Delete unnecessary code comments and tighten the comments that remain. |
| `deep-review` | Review a branch in depth and write a consolidated report. |
| `handoff` | Compact the conversation into a handoff file. |
| `write-plan` | Write a self-executing implementation plan. |
| `write-skill` | Write a new skill. |
| `write-spec` | Write a specification. |
