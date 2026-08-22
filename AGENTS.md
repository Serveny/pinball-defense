## Rules

- Do not add comments, unless if critical context/intent is genuinely lost without it; never explain what code already reveals.
- **Rely on documentation**: Do not read the code of the dependencies in first place, only if something documented is not working or no answer is found in the documentation
- use worktrees unless told otherwise

## Tips

- Cut big tasks into many small ones and use parallel sub agents if possible
- You are working with cutting edge technology with many changes. Always check the documentation and internet for current best practice

## Worktree Workflow

Use `.worktrees/slot-<n>` to avoid Rust recompilation costs.

- If no slot exists or no one is free, create one (count `n` up).
- If you found a free slot, set it to main branch state
- All worktrees share the main repo's `target/` via `CARGO_TARGET_DIR`, so
  external dependencies are compiled only once and reused across slots.

1. **Claim**: Find an idle slot:

```bash
if [ ! -f .worktrees/slot-1/.agent.lock ] || ! kill -0 $(cat .worktrees/slot-1/.agent.lock) 2>/dev/null; then
  echo $$ > .worktrees/slot-1/.agent.lock
  cd .worktrees/slot-1
  export CARGO_TARGET_DIR="$(cd "$(git rev-parse --git-common-dir)/.." && pwd)/target"
fi
```

2. **Work**:

```bash
git checkout -B <feature-branch>
cargo check
```

3. **Release**:

```bash
rm -f .agent.lock
```

## graphify

Knowledge graph at `graphify-out/graph.json` (scoped to `src/`).

- For codebase questions, query the graph first: `graphify query "…"`, `graphify path "A" "B"`, `graphify explain "X"` — before reading source.
- After code changes, run `graphify update src` (AST-only, free).
