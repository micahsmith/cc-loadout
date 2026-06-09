---
name: brainstorm
description: Interview the user to reach alignment on design and tasks before any work begins. Use for ideation or creative work or when the user wants to brainstorm or solidify requirements.
---

Interview the user relentlessly until **total** alignment is achieved. We aim to create a complete
and unambiguous picture of what is to be built or what the requirements are. Leave no unexamined
assumptions. The conversation will be the encoding of this shared understanding.

## Pre-Interview

Explore before the interview. Read relevant code, documentation, and recent commits. Try to answer
questions from the codebase before asking the user.

## Interview

- **Ask one question at a time:**
  - If a topic can be decomposed, it MUST be decomposed.
- **State options:** 
  - For each topic, provide a list of concrete options available when you can.
  - Open ended questions MAY be asked when the space is genuinely open.
  - You MAY use `AskUserQuestion`.
  - If the topic under discussion implies trade-offs, explicitly state the trade-offs.
  - You MAY recommend an answer but do not need to do so.
- **Work through decisions in order:**
  - Address decisions that have broader implications first and keep minor decisions to the end.
  - Only resolve decisions that change the outcome. We want relentlessness in alignment, but do not
    need total exhaustive interrogation.
- **The user ends the interview:**
  - You MAY inform the user when you think the interview is done.
  - The user can decide to end the interview whenever.
  - Once the interview is over, provide an overview of where things stand. Note anything that
    remains uncertain or unsettled.

## Knowing when you are done

You are aligned when you can state the purpose, the approach, the scope, and the success criteria,
and the user agrees with all of it. Before you conclude, surface the choices worth weighing: propose
2-3 approaches with trade-offs and your recommendation, so the direction is chosen rather than
assumed.

When you believe you are there, play back the full picture in your own words and ask the user to
confirm or correct it. Alignment is their agreement, not your assumption.

## What happens next

Nothing is mandated. The shared understanding is the deliverable. Depending on what the user wants,
it may flow into a written spec or design doc, an implementation plan, direct work, or simply sit as
agreed context. Ask where they want to take it; do not assume a structure, a file location, or a
next skill.
