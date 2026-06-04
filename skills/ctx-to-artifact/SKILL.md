---
name: artifact
description: Compact the conversational context into a persisted artifact.
argument-hint: "What will you use this artifact for?"
disable-model-invocation: true
---

Create a file that condenses the current context into a standalone artifact. A *fresh* agent — one
with no access to this conversation — should be able to read it and continue the work without
guessing.

If an argument is provided, treat it as a description of the artifact's intended use and tailor the
content toward that goal.

## Where to save it

1. If the workspace has a designated scratch or temp folder that is git-ignored, save the file
   there.
2. Otherwise, fall back to a temp file via `mktemp`.

## Filename

Name the file `artifact-<short-summary>-<date>.md`, where:

- `<short-summary>` is a few kebab-case words describing the content (e.g. `auth-refactor`).
- `<date>` is today's date as `YYYY-MM-DD`.

Apply this name in both cases above. For the `mktemp` fallback, pass it as the template, e.g.
`mktemp -t artifact-auth-refactor-2026-06-04-XXXXXX.md`.

## What to write

Prefer references over inlined content:

- Cite files and locations as `path:line` (include line numbers for edits in larger files).
- Cite commits by `git` SHA.
- Cite external sources by URL.

Structure the artifact with these sections (omit any that would be empty):

- **Goal** — what we're ultimately trying to achieve, in 1–2 sentences.
- **Current state** — what's done and what's in progress; reference commits by SHA.
- **Key locations** — files, paths, URLs, and symbols the next agent needs, each with a one-line
  note on why it matters.
- **Decisions & constraints** — choices already made and *why*, so they aren't relitigated.
- **Next steps** — the concrete remaining actions, ordered.
- **Gotchas** — non-obvious traps, failed approaches, and things that look wrong but aren't.

When finished, report the artifact's path back to the user.
