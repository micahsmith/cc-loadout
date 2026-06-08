---
name: write-skill
description: Create a new skill. Provides guidance on progressive disclosure and bundled resorces.
---

## Process

1. **Gather Requirements**:
   - Intention: what task is the user solving with the skill?
   - Scope: what are the core use cases?
   - You MAY skip this step if the conversation already contains requirements.

2. **Write Skill**:
   - Draft SKILL.md and any needed additional files.
   - If the SKILL.md exceeds 500 lines, extract content by domain into reference files.
     - For instance, if a significant portion of the SKILL.md provides information about code standards, extract them into a STANDARDS.md file.
     - Keep simple scripts (lte 12 lines) inline, otherwise write to the `scripts` directory.
   - When deterministic behavior is desired, use a script.
   - Use simple language. Avoid jargon unless it is needed for clarity.
   - Be concise. Write more ONLY when it provides clarity.

3. **Review**:
   - Provide the user with your draft and get feedback.
   - Iterate using the guidance from (2) and (3) until the user approves.

## Structure

### Directory

Use the following for the directory structure of the SKILL.md and any supporting files:

```
skill-name/
├── EXAMPLES.md     # Optional
├── <REFERENCE>.md  # Optional
├── SKILL.md        # Required
└── scripts/        # Optional
    └── <SCRIPT>.sh
```

### Description

The description must make it plain when and only when the skill should be used. Explain what the
skill does and when it should be used. Note if there are any special triggers.

You MUST conform to these criteria:
- 1024 character maximum.
- 3 sentence maximum.
- Use simple language.
- Be concise.

### Scripts

Scripts MUST be used when deterministic and repeatable operations are needed in the course of
executing the skill. Assume scripts will be run across multiple platforms.
