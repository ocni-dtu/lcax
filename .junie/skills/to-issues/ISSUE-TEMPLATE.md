## Parent

A reference to the parent issue on the issue tracker (if the source was an existing issue, otherwise omit this section).

## What to build

A concise description of this vertical slice. Describe the end-to-end behavior, not layer-by-layer implementation.

Avoid specific file paths or code snippets — they go stale fast. Exception: if a prototype produced a snippet that encodes a decision more precisely than prose can (state machine, reducer, schema, type shape), inline it here and note briefly that it came from a prototype. Trim to the decision-rich parts — not a working demo, just the important bits.

## Key interfaces

Describe interfaces, types, and behavioral contracts that the agent should look for or modify.
- `TypeName` — what needs to change and why
- `functionName()` return type — what it currently returns vs what it should return
- Config shape — any new configuration options needed

## Acceptance criteria

- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Out of scope

State what is out of scope. This prevents the agent from gold-plating or making assumptions about adjacent features.
- Thing that should NOT be changed or addressed in this issue
- Adjacent feature that might seem related but is separate

## Blocked by

- A reference to the blocking ticket (if any)

Or "None - can start immediately" if no blockers.
