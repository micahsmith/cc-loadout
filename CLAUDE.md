## Overview

This repository is a library of skills for AI agents. There is no build/test capability available.

## Style

Skills and documentation are prose instructions intended for AI agents. Use the following guidance
for writing:

- Be clear. You MUST avoid excess jargon. Prefer simple and legible prose.
- Be concise. ONLY add words in service of clarity; cut out any other excess.
- Use RFC directives in skills and documentation. The modality of an instruction MUST be
  expressed using all capital terms like 'MUST', 'MUST NOT', 'SHOULD', 'MAY', etc.

## Authoring Hooks

Hooks that ship as part of the plugin are located in `hooks`.

## Authoring Skills

- Each skill is located at `skills/<name>/SKILL.md`. Skills MAY use progressive disclosure in order
  to reduce complexity.
- Skills that write files MUST try to resolve save location using the same convention. These
  skills MUST use `mktemp` or a system temp directory ONLY inside a bundled script. Instructions
  on file path resolution MUST be duplicated inline for any skill that writes files. Any change to
  this convention MUST be synced across all skills. The order for resolving save location is as
  follows:
  1. an output-specific directory established by convention (e.g., if the repository has a `specs/`
     directory then specs MUST be written there)
  2. an existing scratch directory (e.g., `tmp/`, `temp/`, `scratch/`, especially if git-ignored)
  3. the repository root or the current working directory if not in a repository
