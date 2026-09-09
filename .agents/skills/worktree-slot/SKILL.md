---
name: worktree-slot
description: Persistent worktree slots for this repo. Use whenever starting any coding task — claim a slot under .worktrees via .agents/skills/worktree-slot/slot.sh, work there, release it only once its work is fully merged into main. Slots keep target/ warm (no full Rust rebuilds).
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
3. `release <slot>` — refuses unless ALL work in the slot is already in `main`. That means: no uncommitted changes (tracked or untracked) AND no unmerged commits (every `slot-N` commit merged/cherry-picked into `main`). Verify with `git -C <slot> log main..slot-N` — must be empty. Releasing with unmerged commits is a bug, not a valid state: the next `claim` runs `reset --hard main` on the slot, which silently destroys that work.

## Rules

- Release is safe only when the slot is fully clean and fully merged. Never release with any unmerged commit or any uncommitted change left in the slot.
- Never delete a slot directory or its `target/`.
- Never call `git worktree remove` on a slot; `release` never does either.
- If `claim` fails with "branch already exists" or "no free slot", run `list` and check `git branch --list 'slot-*'` for stale branches (`git branch -D slot-N` when its worktree is gone and its work is merged or abandoned).