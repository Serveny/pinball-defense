#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: slot.sh claim <task-slug> | release <slot> | list

claim   Rent the next free .worktrees/slot-N, reset it to main, mark it with the slug.
        Never delete a slot directory: its target/ is the warm build cache.
release Finish a task: save your work first (commit or merge), then run.
        Removes only the .task marker; refuses if the slot has uncommitted changes.
list    Show slots: free or busy (with slug and age).
EOF
}

cd "$(git rev-parse --path-format=absolute --git-common-dir)/.."
mkdir -p .worktrees

slot_dirs() {
  ls -d .worktrees/slot-* 2>/dev/null || true
}

MAX_SLOTS=4

cmd="${1:-}"; shift || true
case "$cmd" in
  claim)
    slug="${1:?usage: slot.sh claim <task-slug>}"
    exec 9>.worktrees/.claim.lock
    flock 9
    for d in $(slot_dirs); do
      [ -f "$d/.task" ] && continue
      echo "claiming $d"
      git -C "$d" reset --hard main
      git -C "$d" clean -fd
      printf '%s %s\n' "$slug" "$(date -Iseconds)" > "$d/.task"
      echo "$d"
      exit 0
    done
    n=$(slot_dirs | wc -l)
    if [ "$n" -lt "$MAX_SLOTS" ]; then
      d=".worktrees/slot-$((n + 1))"
      echo "creating $d (one-time full rebuild cost)"
      git worktree add "$d" -b "slot-$((n + 1))" main
      printf '%s %s\n' "$slug" "$(date -Iseconds)" > "$d/.task"
      echo "$d"
      exit 0
    fi
    echo "no free slot; run ./slot.sh list" >&2
    exit 1
    ;;
  release)
    slot="${1:?usage: slot.sh release <slot-path-or-N>}"
    case "$slot" in
      */*) d="$slot" ;;
      [0-9]*) d=".worktrees/slot-$slot" ;;
      *) d=".worktrees/$slot" ;;
    esac
    if [ ! -f "$d/.task" ]; then
      echo "$d has no .task marker" >&2
      exit 1
    fi
    if [ -n "$(git -C "$d" status --porcelain --untracked-files=no)" ]; then
      echo "refusing: $d has uncommitted changes. Commit/merge your work first, then release again." >&2
      echo "  discard on purpose? git -C '$d' reset --hard main && git -C '$d' clean -fd && rm '$d/.task'" >&2
      exit 1
    fi
    rm "$d/.task"
    echo "released $d"
    ;;
  list)
    for d in $(slot_dirs); do
      if [ -f "$d/.task" ]; then
        read -r slug ts _ < "$d/.task"
        echo "busy: $d  task=$slug  since=$ts"
      else
        echo "free: $d"
      fi
    done
    ;;
  *)
    usage >&2
    exit 1
    ;;
esac