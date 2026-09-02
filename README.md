# cc-loadout

Skills and hooks library for Claude Code and other agents. 

## Installation

The easiest installation method is just to prompt (as it doesn't require specific ssh or git
configuration):

```text
Fetch https://github.com/micahsmith/cc-loadout over HTTPS, install it as a plugin marketplace, and
install the `code-skillset` plugin.
```

If `ssh` is available or `git` is setup to use HTTPS, these commands work:

```sh
/plugin marketplace add micahsmith/cc-loadout
/plugin install code-skillset@cc-loadout
```

## Hooks

| Hook | Purpose |
|------|---------|
| `technical-writing` | Rules for technical writing that aim to improve comprehension and clarity. |

## Skills

| Skill | Purpose |
|-------|---------|
| `bootstrap` | Apply preferred global settings. |
| `brainstorm` | Interview session to achieve alignment on design and requirements prior to work. |
| `deep-review` | Comprehensive code review with a consolidated report artifact. |
| `handoff` | Compact the conversation into a handoff artifact. |
| `write-adr` | Record an architecture decision as a numbered, indexed ADR. |
| `write-plan` | Write a self-executing implementation plan. |
| `write-spec` | Write a specification artifact. |
| `write-skill` | Write a new skill. |
