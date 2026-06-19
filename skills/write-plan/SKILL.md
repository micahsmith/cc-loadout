---
name: write-plan
description: Create an implementation plan. The created plan is self-executing, in that it encodes instructions on how to execute and a completion sentinel in order to facilitate use with a loop. Use this to break down a specification or design into concrete chunks of work.
disable-model-invocation: true
---

Create an implementation plan from a provided specification (or design provided in the
conversation). Write the plan as an ordered set of tracer-bullet slices. The plan encodes **how** it
is to be implemented so that nothing beyond the plan is required to be used in agent loops like
`/goal`. Thus: every plan must encode **decisions**, **slices**, **tests**, and an **end
condition**.

Plans MAY include code and implementation details, but ONLY when they are absolutely necessary to
successful execution of the specification/design. Most implementation SHOULD be left to the agent
that will execute the plan. The executor will write the code; the plan is intended to constrain
choices that are costly to get wrong or that will diverge from alignment on design.

The plan MUST be include instructions for execution in an agentic loop (`/goal`). There SHOULD be NO
assumption that further context or skills will be made available when the plan is implemented. Since
the loop only reads the conversation transcript, every plan will end with a literal completion
sentinel to signal completion.

A full specification file is preferable as input, but not required. If there is not enough context
to create a plan, use `/brainstorm` and `/write-spec` first.

## Process

1. **Explore.** Read any relevant code, tests, and documentation. This exploration will need to be
   deep, since you need to understand the layers of code that the work crosses, current standards
   and patterns to follow, and how tests are written and run. You MUST understand sufficiently to be
   able to define slices, ordering and dependencies, decision anchors, and concrete test cases.
2. **Draft Slices.** Decompose the work into tracer-bullet slices (see "Slices").
3. **Interview & Iterate.** Present the draft. Confirm with the user before proceeding further. To
   resolve gaps, ask one question at a time.
4. **Write.** Create the plan file (see "Output").
5. **Review.** Scan the draft and fix problems inline (see "Self-Review").
6. **Report.** Report the plan file path, a short summary of its contents.

## Plan Structure

A plan has three parts:

1. **Header.** The header includes the feature name, a one-sentence statement of the goal, a link to
   the specification (if it exists), and the slice index with ordering.
2. **Execution Protocol.** The standardized block (see "Execution Protocol").
3. **Slices.** The work broken down into tracer-bullet style chunks.
4. **Completion Sentinel.** The last part of the plan is the completion report template (see
   "Completion Sentinel").

### Execution Protocol

Write this block into every plan verbatim. It provides the agent with guidance on how to execute the
plan within a loop.

> ## How To Execute This Plan
>
> You are to implement the plan presented here. Do your work in a worktree. Dispatch each slice of
> work to a fresh subagent if possible. Prove each slice with surfaced test output. Stop when you
> emit the completion report.
>
> 1. **Worktree.** Do all work in a git worktree (if worktrees are available) named after this plan
>    file.
> 2. **Sequential Slices in Order.** Follow the ordering for slices. You MUST NOT start a slice if
>    the slices it is blocked by have not finished verification. Independent slices MAY run in
>    parallel ONLY if each runs in its own worktree. Independent worktrees MUST be merged back into
>    the main worktree once complete.
> 3. **Delegate Slices to Subagents.** Each subagent should receive ONLY this execution protocol and
>    details on the slice it is implementing (the plan file identifier and the slice identifier).
>    Keep context windows small.
> 4. **TDD.** Use red/green/refactor. Demonstrate failure of the test first, write the minimum
>    implementation to make it pass, and then demonstrate success. Lastly, refactor and iterate.
> 5. **Surface Proof.** Each subagent MUST report test results and the checkpoint status for the
>    slice. You MUST include this proof in the main conversation so that completion is evaluable.
> 6. **Terminate.** When every slice checkpoint is verified, run the full test suite, exhibit that
>    all tests pass, and print the completion report verbatim.

### Slices

Each slice is a thin vertical slice of the application that can be independently executed (each
slice is a tracer-bullet). Write slices in order so that each builds on its predecessors.

If the codebase needs to be softened before adding feature slices, include a **prefactoring** slice
to facilitate easier work for future iterations. ONLY include this prefactoring slice when high
value. 

Each slice MUST contain:

- **Title.** The title should describe the end-to-end behavior the slice delivers.
- **Blocked By.** Slices that must be implemented before work can proceed, else "None".
- **What to Build.** Behavior across the layers the slices intersects (e.g. schema, API, UI). The
  constraints and invariants that MUST be followed. ONLY include code if it is the simplest way to
  express the constraints and invariants.
- **Decision Anchors.** Interfaces, signatures, and contracts that the implementation must satisfy.
- **Test Cases.** One test case for each behavior. Tests must cover happy paths, edge cases, and
  error cases. Each test case states a **name**, **setup/preconditions**, **action** with concrete
  inputs, and the **expected observable result** with concrete values. Do NOT write test code. See
  "Tests" for additional instructions.
- **Checkpoint.** The observable state when the implementation is done, stated so that the executor
  can surface it as proof of completion (which tests have passed and which behavior is demoable).

### Completion Sentinel

End the plan with the report the executor prints only when all slices are verified. This sentinel
allows loops to detect completion:

> ## Completion report
>
> Print this ONLY when all slice checkpoints are verified:
>
> - One line per slice: its title, ✅, and the checkpoint proof.
> - The full test suite command and its output.
> - Final line verbatim: `PLAN COMPLETE: plan-<date>.md`

## Output

### Save Location

Try to resolve the base directory for the plan in this order:

1. **Specification Directory.** If a specification for this work exists, write the plan into the
   SAME directory as the specification.
2. **Conventional Specification Directory.** If a directory conventionally used for plans or
   specifications already exists (`plans/`, `specs/`, `prds/`, or similar), use it.
2. **Scratch Directory.** Otherwise, if an existing scratch directory is present at the repository
   root (`tmp/`, `temp/`, `scratch/`, or similar, especially if git-ignored), use it.
3. **Fallback.** Otherwise, use the repository root, or the current working directory if not in
   a repository.

If there is no specification file to be sibling to the plan, create a new directory whose name is at
most four words describing the work to be done in kebab-case, and write the plan there.

Do NOT use `mktemp` or any system temp directory.

### Filename

Create the file named `plan-<date>.md`, where `<date>` is the output of `date +%F`.

### Splitting

Default should be a single file. If the plan grows very large (>1500 lines), split the plan into
a series of sibling files named `plan-<date>-<slice>.md` and keep the `plan-<date>.md` as the
**index**. The index holds the header, the execution protocol, the slice list with ordering, and the
completion report. It also includes pointers to each other plan file. Do NOT pre-split small plans.

## Self-Review

Before the final report, scan the draft and fix problems inline:

- Placeholders or TODOs.
- Missing test cases.
- Test cases without concrete and observable results.
- Missing edge/error cases.
- Cyclic or contradictory slice ordering.
- Decision anchors that contradict the specification.
- Missing execution protocol or completion sentinel.

Additional review will be conducted by the user conversationally.

## Tests

Tests MAY be either unit or integration tests. Tests MAY test either behavior or properties. Tests
should be written to guide the implementation: excessive fixtures and complicated mocking often
indicate that the unit under test needs to be refactored to simplify the test.

Do NOT keep tests that are written to confirm that behavior has changed. Tests should be written to
verify current state and behavior. Any test used to confirm that behavior has changed compared to
previous states of the codebase should be removed before registering completion.
