---
name: handoff
description: Compact the conversational context into a persisted handoff file.
argument-hint: "What will you use the handoff for?"
disable-model-invocation: true
---

Create a file that condenses the current context into a standalone file. An agent with an empty
context should be able to read the handoff and continue the work without additional inputs.

If an argument is provided, treat it as a description of the handoff's intended use and tailor the
content toward that goal.

## Save location

1. If the workspace has a designated scratch or temp folder (for instance, if there is a git-ignored
   tmp folder at the workspace root, that is strong evidence it is a scratch folder), save the file
   there.
2. Otherwise, use `mktemp` to create the file.

## Filename

Name the file `handoff-<short-summary>-<date>.md`, where:

- `<short-summary>` is at most three words in kebab-case describing the content.
- `<date>` is obtained from the shell with `date +%F` (which yields `YYYY-MM-DD`).

Apply this name regardless of save location. For the `mktemp` fallback, pass it as the template,
e.g., `mktemp -t "handoff-auth-refactor-$(date +%F)-XXXXXX.md"`.

## What to write

Prefer references over inlined content:

- Cite reference files (ARDs, PRDs, specs, etc.) using the file path without a line number.
- Cite larger files or files where more targetted attention is needed using a line number:
  `path:line`.
- Cite commits by `git` SHA.
- Cite external sources by URL.

Structure the handoff with these sections (omit any that would be empty):

- **Goal**: a 1-3 sentence summary of purpose which takes into account both the current conversation
  and the user provided argument (if provided).
- **Current State**: what has been done and what is unfinished.
- **Key Locations**: files, paths, URLs, and symbols the next agent needs. Provide a one-line note
  explaining why the location is relevant to the conversation.
- **Decisions & Constraints**: choices already made and why they were made. We want to clarify
  intention and avoid relitigating previous decisions.
- **Plans**: the concrete steps identified in context that haven't yet been finished (if any).
  Provide these in order if possible.
- **Gotchas**: non-obvious traps, failed approaches, and things that look wrong but aren't.

Report the handoff's path back to the user.
