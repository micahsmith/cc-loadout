---
name: deep-review
description: Runs a deep review on a branch along four dimensions (security, performance, correctness, style) and creates a consolidated report. Use when the user asks to deeply review or audit a branch without disturbing the working tree.
argument-hint: "[target] [base] — default behavior compares `HEAD` against `main`/`master`"
disable-model-invocation: true
---

Run a thorough code review of the changes on the branch against the base; then write a single
consolidated report. The review is read-only: it MUST NOT switch branches or touch the user's
working tree, so that the review process will not interfere with other concurrent work.

The code review will be done by four separate reviewers, each of which is focused on a single aspect
(security and privacy, performance, correctness, and style). They explore the changes and the code
freely, then each writes a focused report on its findings. Lastly, the individual reports are
consolidated into one final report for review.

## Arguments

The arguments indicate the branch under review (`TARGET`) and the branch to compare against
(`BASE`). Accept either positional references or plain language ("review this branch against the
`dev` branch").

- No arguments: `TARGET` is the current branch. `BASE` is `main`, `master`, or whichever branch
  serves as the primary branch. If you cannot find an obvious branch candidate, ask the user.
- One argument: the argument is `TARGET`. Resolve `BASE` using the same rules as above.
- Two arguments: the first argument is `TARGET` and the second is `BASE`.

By default, fetch the latest from the remote. Pass `--no-fetch` to the prep script when the user
asks to skip the fetch or is working offline.

## Steps

### 1. Prepare

Run the prep script with the resolved references:

```sh
bash scripts/prepare-review.sh [--no-fetch] [TARGET] [BASE]
```

The script will resolve the references, compute the merge-base, and decide whether the review can be
done in place or in a throwaway worktree. It returns a summary block that delineates the work to be
reviewed. The summary block includes:

- `WORKDIR`: where reviewers undertake the review (the worktree or the repo root if the review is
  "in place").
- `TARGET_SHA`: the commit SHA containing the changes to be reviewed (`TARGET`).
- `MERGE_BASE`: the `BASE` against which `TARGET` is compared.
- `DIFF_CMD`: the diff command reviewers should run.
- `CLEANUP`: the command to cleanup the worktree (if present) after the review is finished.
- The diffstat and changed-file list.

If the script fails, report the cause to the user.

### 2. First Pass Summary

Spawn one inexpensive agent to read the diffstat and changed-file list. The agent MUST write a 3-6
sentence plain language summary of the changes. Pass this summary to all reviewers as shared initial
context.

### 3. Parallel Fan-out

Call the four reviewers to run in parallel (the same message passed to four `Agent` calls). DO NOT
paste the diff into their prompts. Provide each reviewer with the same context and with its
individual focus:

- `WORKDIR`, `MERGE_BASE`, `TARGET_SHA`, `DIFF_CMD`, diffstat, and changed-file list.
- The first pass summary from step 2.
- Instructions on issue report format and individual focus.

Each reviewer MUST run the `DIFF_CMD` itself, explore under `WORKDIR`, and return with a report.
Always launch all four reviewers regardless of how small the diff is. A reviewer MAY report "No
issues found".

The four focuses:
- **Security and Privacy**
  - Leaked secrets
  - Injection threats
  - Authentication/authorization failures
  - Unsafe input handling
  - Unsafe handling of sensitive data
- **Performance**
  - Needless work or complexity
  - N+1 queries
  - Wasteful memory allocations or I/O ops
- **Correctness**
  - Errors in logic
  - Broken edge cases
  - Race conditions
  - Incorrect or incomplete error handling
  - Unintended breaks in backwards compatibility
  - Mismatch with intent
- **Style**
  - Non-idiomatic use of language or framework
  - Violation of existing codebase patterns
  - Illegible or hard to decipher code blocks
  - Excessive or trivial comments on well-written code
  - Violation of any rules provided in `AGENTS.md`, `CLAUDE.md` and the like

Rule for reviewers: report ONLY issues pertinent to your focus. However, if an issue MIGHT pertain
to your focus, report it even when it also belongs to another focus. Several reviewers MAY report
the same issue.

### 4. Consolidated Report

Collate, deduplicate, and organize the four reports. Do **NOT** overrule a reviewer: every issue
a reviewer flags MUST reach the final report. Merge an issue raised by several reviewers into
a single entry that names every ID which reported it.

### 5. Final Report and Summary

Write the full report to a file named `review-<target>-<date>.md`, where `<date>` is the output
of `date +%F`. Resolve the base directory for the file as follows:

1. **Reviews Directory.** If the repository has a directory conventionally used for reviews
   (`reviews/` or similar), use it.
2. **Scratch Directory.** Otherwise, if an existing scratch directory is present at the repository
   root (`tmp/`, `temp/`, `scratch/`, or similar, especially if git-ignored), use it.
3. **Fallback.** Otherwise, use the repository root, or the current working directory if not in
   a repository.

Do NOT use `mktemp` or any system temp directory.

Report the file location to the user with a short summary of findings: the count of issues by
severity, and the top-level blocking issues. Do NOT reproduce the entire report unless it contains
three or fewer issues.

### 6. Cleanup

If the prep script returned a `CLEANUP` line, run that command to remove the worktree.

## Issue Format

Every reviewer MUST use this standard format for every issue. Write each issue in plain and simple
language. You MAY use technical terms that developers commonly understand, but you MUST avoid
jargon. A reader MUST be able to understand each issue from the report alone.

Use this format:


```
### [SEC-1] <one-line title>
- **Severity:** Blocking | Major | Suggestion | Style
- **Confidence:** High | Medium | Low
- **Location:** `path/to/file.js:120-134`

<code snippet>

**Issue Statement**: What is the problem and why is it a problem.

**Proposed Solution:** How the problem could be resolved, and why the proposed solution fixes it.

**Confidence Note**: (required when Confidence is Medium or Low) Why confidence was not High and
what ambiguity or intent needs to be resolved in order to confirm status.
```

Use one ID prefix per focus:

- `SEC` for security and privacy
- `PERF` for performance
- `COR` for correctness
- `STY` for style

Group issues by severity, blocking first. Open each report with a 2-3 sentence summary.

## Report Structure

The final report MUST use this format:
1. **Summary**: a table of issue counts per severity per focus followed by the list of blocking
   issues.
2. **Issues by Severity**: (Blocking → Major → Suggestion → Style). Each issue keeps its ID and
   focus tag. A merged issue MUST list the ID from each reviewer and combine their content.
