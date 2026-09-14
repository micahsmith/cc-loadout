# Code Comment Style Guide

Version 2.

Apply these rules to every code comment you write or edit. Defer to the user if they ask for a
different style.

Code states what happens. Comments state what code cannot. Default to no comment.

## Inline Comments

Default to no comment. An inline comment MAY exist only as a reason, a constraint, or a warning.

Apply both of the following tests before writing a comment. Both tests are to be run
hypothetically: neither should appear in the output.

- **Deletion:** Apply the deletion test to each sentence. Delete the sentence and name the fact
  that is now lost. If you cannot name the fact that is now absent from the file, the sentence
  should be deleted.
- **Opener:** Read the comment as opening with "Because...", "Requires...", or "Careful:...". If no
  coherent reading is possible with the opener, the comment restates code and should be deleted.

A comment that refers to another system, file, or measurement MUST name it.

Do NOT write to the reader of a diff. Delete `// now also handles null` and `// as you requested`.

## Doc Comments

Default to no doc comments. Doc comments serve consumers of a library or utility API: callers who
use a declaration without reading it. They do NOT belong on everything public.

Match the doc-comment density of sibling declarations. Default to none.

Every sentence MUST state something absent from the name, parameters, and return type.
