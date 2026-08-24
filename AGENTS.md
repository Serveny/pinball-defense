## Rules

- Do not add comments, unless if critical context/intent is genuinely lost without it; never explain what code already reveals.
- **Rely on documentation**: Do not read the code of the dependencies in first place, only if something documented is not working or no answer is found in the documentation
- ALWAYS use a git worktree (`./.worktrees`) for coding tasks
- query graphify for codebase questions

## Tips

- Cut big tasks into many small ones and use parallel sub agents if possible
- You are working with cutting edge technology with many changes. Always check the documentation and internet for current best practice

## graphify

Knowledge graph at `graphify-out/graph.json` (scoped to `src/`).

- For codebase questions, query the graph first: `graphify query "…"`, `graphify path "A" "B"`, `graphify explain "X"` — before reading source.
- After code changes, run `graphify update src` (AST-only, free).

## Documentation

- When the user says "update adm": Save notable hard-won knowledge from this session — retrieved dependency docs, pitfalls, shortcuts, non-obvious facts. Skip if nothing notable was learned.

For each item, pipe the markdown body via stdin (one command per item):

```
adm info add pinball-defense "<name-of-info>"
```
