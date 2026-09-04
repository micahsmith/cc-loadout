---
name: handoff
description: Compact the conversational context into a persisted handoff file.
argument-hint: "What will you use the handoff for?"
disable-model-invocation: true
---

Create a file that condenses the current context into a standalone file. An agent with an empty
context MUST be able to read the handoff and continue the work without additional input.

If an argument is provided, treat it as a description of the handoff's intended use and tailor the
content toward that goal.

## Save Location

Resolve the base directory for the file as follows:

1. **Handoffs Directory.** If the repository has a directory conventionally used for handoffs
   (`handoffs/` or similar), use it.
2. **Scratch Directory.** Otherwise, if an existing scratch directory is present at the repository
   root (`tmp/`, `temp/`, `scratch/`, or similar, especially if git-ignored), use it.
3. **Fallback.** Otherwise, use the repository root, or the current working directory if not in
   a repository.

Do NOT use `mktemp` or any system temp directory.

## Filename

Name the file `handoff-<short-summary>-<date>.md`, where:

- `<short-summary>` is at most three words in kebab-case describing the content.
- `<date>` is obtained from the shell with `date +%F` (which yields `YYYY-MM-DD`).

## What to write

Prefer references over inlined content:

- Cite reference files (ADRs, PRDs, specs, etc.) using the file path without a line number.
- Cite larger files or files where more targeted attention is needed using a line number:
  `path:line`.
- Cite commits by `git` SHA.
- Cite external sources by URL.

Structure the handoff with these sections (omit any that would be empty):

- **Goal**: a 1-3 sentence summary of purpose. Base it on the current conversation and on the
  argument, if the user provided one.
- **Current State**: what has been done and what is unfinished.
- **Key Locations**: files, paths, URLs, and symbols the next agent needs. Provide a one-line note
  explaining why the location is relevant to the conversation.
- **Decisions & Constraints**: choices already made and why they were made. Stating them clarifies
  intent and stops the next agent from relitigating settled decisions.
- **Plans**: the concrete steps identified in context that haven't yet been finished (if any).
  Provide these in order if possible.
- **Gotchas**: non-obvious traps, failed approaches, and things that look wrong but aren't.

Report the handoff's path back to the user.
