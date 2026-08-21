## Rules

- Do not add comments, unless if critical context/intent is genuinely lost without it; never explain what code already reveals.

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
