---
name: worktree-slot
description: Use for coding tasks in this repo. Claim a persistent worktree slot with a feature-named branch, preserving its Cargo build cache. Commit there; merge or release only when the user requests it.
---

# Persistent worktree slots

Work in `.worktrees/slot-N`, with a new branch for each task, defaulting to `feat/<task-slug>`. The directory stays fixed so its `target/` and ignored files survive reuse. Cargo can reuse compatible build artifacts; dependency, toolchain, profile or compiler-flag changes can still require rebuilds. Each slot has its own cache and checkout, so additional slots cost disk space and begin with a cold cache.

There is no slot limit or configuration. `claim` reuses a free slot or creates another if all are occupied. The user controls how many tasks run concurrently. Slot management uses Bash, Git and standard Linux tools; Python is only used by the maintenance tests.

## Workflow

1. Before editing, claim a slot using a short feature slug. For follow-up work on an existing task, run `list` and continue in its occupied slot instead of claiming another one.

   ```bash
   slot_path=$(.agents/skills/worktree-slot/slot.sh claim microwave-effects)
   cd "$slot_path"
   ```

   `claim` prints one absolute path to stdout and diagnostics to stderr. It creates `feat/microwave-effects` from **local** `main`. For a different branch name, use `claim microwave-effects fix/microwave-effects`. Existing branch names are refused, never overwritten. Reusing a slot requires a clean worktree and integrated commits; a missing `.task` marker alone does not make it safe.

2. Do the work and commit on the feature branch. Report the branch, slot path, changes and validation. Keep the slot occupied for review and follow-up work.

3. Merge or release only when explicitly requested by the user. `release` requires no uncommitted changes (including untracked files) and all work integrated into `main`. It accepts reachable commits or cherry-picked patches compared with whitespace preserved (`git patch-id --verbatim`). Squash merges, empty commits and unique merge commits can fail this conservative check; inspect the history and report the blocker rather than deleting markers or forcing a reset. Release detaches the slot at `main`, removes `.task`, and preserves the feature branch and build cache.

## Commands

```bash
.agents/skills/worktree-slot/slot.sh claim <task-slug> [branch]
.agents/skills/worktree-slot/slot.sh release <slot>
.agents/skills/worktree-slot/slot.sh list
```

`release` accepts a slot number, `slot-N`, an absolute slot path, or `.worktrees/slot-N` relative to the main checkout. The script resolves its repository from its own location; invoking it by absolute path also works from outside the repository.

## Constraints

- Never edit, commit, merge or push in the main checkout without the user's explicit instruction. Never release a slot unprompted. Perform only the requested merge/release actions; do not add pushes or branch cleanup.
- Never delete a slot directory, its `target/`, or a feature branch to make a claim succeed. Never run `git worktree remove` on a slot.
- If claim refuses a dirty or unintegrated free slot, preserve its work and report the problem. Do not use `reset --hard`, `clean`, or delete `.task` to bypass checks.
- Ignored files persist, including saves and local runtime state. Check them when they affect a test or preview; they are not automatically synchronized with the main checkout.

When changing the script, run `python3 .agents/skills/worktree-slot/tests/test_slot.py`; it exercises the lifecycle in temporary Git repositories without touching real slots.
