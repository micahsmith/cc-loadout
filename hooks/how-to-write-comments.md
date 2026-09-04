# Code Comment Style Guide

Version 1.

Apply these rules to all code comments you write or edit. They cover inline comments and doc
comments. Defer to the user if they ask for a different style.

Code states what happens. Comments state what code cannot. Every comment MUST earn its place. These
two rules pull in opposite directions, but both hold:

- Inline comments should be used sparingly.
- On public declarations, particularly for the API of libraries or utility classes/functions,
  follow the codebase convention.

## Decision Procedure

Before writing a comment, run these steps and stop at the first applicable step.

- **Step 1.** Is the comment site a public declaration of a library API or utility class or
  function and is the codebase convention to write doc comments? Write a doc comment (see "Doc
  Comments").
- **Step 2.** Does the code already express the comment content? Write no comment.
- **Step 3.** Would a better name or a small extraction of code remove the need for a comment? Make
  the code more perspicuous.
- **Step 4.** Does the comment provide a reason, constraint, or warning that cannot be expressed in
  the code itself? Write the comment; otherwise write nothing.

## Positive Guidance

1. Provide reasons, not mechanics.
2. State what the code cannot state. Name contract that cannot be enforced by the code.
3. Put warnings next to the code that needs the warning.
4. Bound claims so that later readers know when they have ceased to be true.

## Negative Guidance

5. Do NOT restate the code. Delete `// increment the counter` above `counter += 1`.
6. Do NOT narrate changes. Describe the code as it is. Delete `// now also handles null` and `//
   added retry logic`.
7. Do NOT address someone involved in implementation. Delete `// as you requested` and `// per your
   feedback`. Comments are for future readers, not for present interlocutors.
8. Most well-written functions need no comments.

## Maintanence

9.  Code changes can invalidate comments. Update or delete invalidated comments in the same change.
10. Incorrect comments are worse than missing comments. Do NOT make any claim you cannot verify.
11. Report stale comments you find outside your changes. Do NOT silently fix comments that may be
    out of scope.

## Doc Comments

12. Do NOT add doc comments if the codebase doesn't use them.
13. Public declarations (especially for the public API or libraries or utilties) are the exception
    to rules regarding sparseness. Consumers should understand what they are using without reading
    the implementation.
14. State the contract. State what guarantees are made, what is returned, what errors may be raised,
    what precautions callers must take, and any surprising behavior.
15. Do NOT merely restate the signature.
16. Do NOT state implementation details. The caller needs to know whether the method is thread-safe,
    not that threads are created using a particular executor.
