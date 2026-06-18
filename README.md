# cc-loadout

Skills library for Claude Code and other agents.

## Installation

### Npx Skills

```sh
npx skills@latest add micahsmith/cc-loadout --all    # install all skills non-interactively
npx skills@latest add micahsmith/cc-loadout -g --all # install all skills globally
```

### Claude Code Marketplace

```sh
/plugin marketplace add micahsmith/cc-loadout
/plugin install code-skillset@cc-loadout
```

Note: `/plugin install` will use the existing `git` config. Generally, this means that it will
attempt to clone the repository via SSH. In turn, the installation may fail in sandboxes or other
environments where `ssh` fails.

## Skills

| Skill | Purpose |
|-------|---------|
| `bootstrap` | Apply preferred global settings. |
| `brainstorm` | Interview session to achieve alignment on design and requirements prior to work. |
| `deep-review` | Comprehensive code review with a consolidated report artifact. |
| `handoff` | Compact the conversation into a handoff artifact. |
| `write-spec` | Write a specification artifact. |
| `write-skill` | Write a new skill. |
