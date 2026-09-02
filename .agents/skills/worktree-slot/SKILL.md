---
name: worktree-slot
description: Persistent worktree slots for this repo. Use whenever starting any coding task — claim a slot under .worktrees via .agents/skills/worktree-slot/slot.sh, work there, release it after committing. Slots keep target/ warm (no full Rust rebuilds).
---

# Slot system — persistent worktrees

For ANY coding task: claim a slot, work there, release when done. Never work in the main checkout, never delete a slot.

Slots persist so `target/` stays warm — a fresh worktree or deleted slot costs a full Rust rebuild (2–6 GB Bevy target). Only `.task` markers and code state change between tasks; `target/` survives.

## Commands

```bash
.agents/skills/worktree-slot/slot.sh claim <task-slug>   # prints the slot path, e.g. .worktrees/slot-1; do all work there
.agents/skills/worktree-slot/slot.sh release <slot>      # accepts N, slot-N, or full path
.agents/skills/worktree-slot/slot.sh list
```

Run from anywhere (inside slots included) — the script resolves the main repo root itself.

## Workflow

1. `claim <task-slug>` — first free slot is reset to `main` (`reset --hard` + `clean -fd`; only task leftovers are discarded, `target/` and ignored files stay) and marked with your slug + timestamp. Slots are created lazily up to 4.
2. Do the work in the printed slot path; commit there on branch `slot-N`.
3. `release <slot>` — refuses with a recovery hint if uncommitted changes remain; otherwise removes only the `.task` marker. Your committed work stays on branch `slot-N` until merged/cherry-picked.

## Rules

- Never delete a slot directory or its `target/`.
- Never call `git worktree remove` on a slot; `release` never does either.
- If `claim` fails with "branch already exists" or "no free slot", run `list` and check `git branch --list 'slot-*'` for stale branches (`git branch -D slot-N` when its worktree is gone and its work is merged or abandoned).