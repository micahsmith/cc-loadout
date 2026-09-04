# cc-loadout

Skills and hooks library for Claude Code and other agents. 

## Installation

The easiest installation method is just to prompt (as it doesn't require specific ssh or git
configuration):

```text
Use HTTPS to fetch https://github.com/micahsmith/cc-loadout and (re-)install as the
`code-skillset` plugin.
```

If `ssh` is available or `git` is setup to use HTTPS, these commands work:

```sh
/plugin marketplace add micahsmith/cc-loadout
/plugin install code-skillset@cc-loadout
```

## Hooks

These guides are injected at `SessionStart` and `SubagentStart`.

| Guide | Purpose |
|-------|---------|
| `how-to-write-comments` | Rules for writing code comments and doc comments. |
| `how-to-write-prose` | Rules for technical writing that aim to improve comprehension and clarity. |

## Skills

| Skill | Purpose |
|-------|---------|
| `bootstrap` | Apply preferred global settings. |
| `brainstorm` | Interview session to achieve alignment on design and requirements prior to work. |
| `deep-review` | Comprehensive code review with a consolidated report artifact. |
| `handoff` | Compact the conversation into a handoff artifact. |
| `write-plan` | Write a self-executing implementation plan. |
| `write-spec` | Write a specification artifact. |
| `write-skill` | Write a new skill. |
