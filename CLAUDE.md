## Overview

This repository is a library of skills for AI agents. There is no build/test capability available.

## Style

Skills and documentation are prose instructions intended for AI agents. Use the following guidance
for writing:

- Be clear. You MUST avoid excess jargon. Prefer simple and legible prose.
- Be concise. ONLY add words in service of clarity; cut out any other excess.
- Use RFC directives. The modality of an instruction should be expressed using all capital terms
  like 'MUST', 'MUST NOT', 'SHOULD', 'MAY', etc.

## Authoring Skills

- Each skill is located at `skills/<name>/SKILL.md`. Skills MAY use progressive disclosure in order
  to reduce complexity.
- New skills should be added to `.claude-plugin/plugin.json` and `README.md`.
