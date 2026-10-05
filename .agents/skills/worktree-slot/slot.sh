#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: slot.sh claim <task-slug> [branch] | release <slot> | list

claim   Reuse a clean, integrated slot or create a new one from local main.
        Default branch: feat/<task-slug>. Existing branches are never overwritten.
        Prints only the absolute slot path to stdout; diagnostics go to stderr.
release Free a clean slot after its work was merged or cherry-picked into main.
        Run only on explicit user request. Preserves the branch and target/.
list    Show slot availability, branch, task and start time.
USAGE
}

fail() {
  echo "refusing: $*" >&2
  exit 1
}

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
common_dir="$(git -C "$script_dir" rev-parse --path-format=absolute --git-common-dir)"
cd -- "$common_dir/.."
root="$PWD"
mkdir -p .worktrees
shopt -s nullglob

slot_dirs() {
  local d
  for d in "$root"/.worktrees/slot-*; do
    [[ "${d##*/}" =~ ^slot-[1-9][0-9]*$ ]] && [ -d "$d" ] && printf '%s\n' "$d"
  done
  return 0
}

check_slot() {
  local d="$1"
  [ "$(git -C "$d" rev-parse --show-toplevel)" = "$d" ] || fail "$d is not a worktree root"
  [ "$(git -C "$d" rev-parse --path-format=absolute --git-common-dir)" = "$common_dir" ] || fail "$d belongs to another repository"
  [ "$(git -C "$d" symbolic-ref --quiet --short HEAD || true)" != main ] || fail "$d has main checked out"
}

patch_ids() {
  git -C "$1" log --format='commit %H' --no-merges --no-ext-diff --no-textconv --no-renames --binary --full-index --color=never -p "$2" |
    git patch-id --verbatim
}

check_integrated() {
  local d="$1" base upstream pending patch commit count=0
  local -A integrated_patches=()
  [ -z "$(git -C "$d" status --porcelain --untracked-files=all)" ] || fail "$d has uncommitted changes; preserve and finish them before reuse"
  if git -C "$d" merge-base --is-ancestor HEAD main; then
    return
  fi
  [ -z "$(git -C "$d" rev-list --merges main..HEAD)" ] || fail "$d has merge commits outside main; review and merge its branch first"
  base="$(git -C "$d" merge-base HEAD main)"
  upstream="$(patch_ids "$d" "$base..main")"
  pending="$(patch_ids "$d" main..HEAD)"
  while read -r patch commit; do
    [ -n "$patch" ] || continue
    integrated_patches["$patch"]=1
  done <<< "$upstream"
  while read -r patch commit; do
    [ -n "$patch" ] || continue
    [ -n "${integrated_patches[$patch]:-}" ] || fail "$d has commits not merged or cherry-picked into main: $commit"
    count=$((count + 1))
  done <<< "$pending"
  [ "$count" -eq "$(git -C "$d" rev-list --count main..HEAD)" ] || fail "$d has commits without comparable patches; review and merge its branch first"
}

cmd="${1:-}"
shift || true
case "$cmd" in
  claim)
    [ "$#" -ge 1 ] && [ "$#" -le 2 ] || fail "usage: slot.sh claim <task-slug> [branch]"
    slug="$1"
    [[ "$slug" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]] || fail "task slug must use lowercase letters, digits and single hyphens"
    branch="${2:-feat/$slug}"
    git check-ref-format --branch "$branch" >/dev/null || fail "invalid branch name: $branch"
    [[ "$branch" != main && "$branch" != slot-* && "$branch" != @* ]] || fail "choose a feature branch name: $branch"
    exec 9>"$root/.worktrees/.claim.lock"
    flock 9
    git show-ref --verify --quiet refs/heads/main || fail "local main branch is missing"
    if git show-ref --verify --quiet "refs/heads/$branch"; then
      fail "branch $branch already exists; use list to find ongoing work or choose another name"
    fi
    d=""
    n=0
    while IFS= read -r candidate; do
      number="${candidate##*-}"
      if (( number > n )); then n="$number"; fi
      if [ -z "$d" ] && [ ! -e "$candidate/.task" ]; then
        check_slot "$candidate"
        check_integrated "$candidate"
        d="$candidate"
      fi
    done < <(slot_dirs)
    if [ -n "$d" ]; then
      echo "claiming $d on $branch" >&2
      git -C "$d" switch --no-overwrite-ignore -c "$branch" main >&2
    else
      d="$root/.worktrees/slot-$((n + 1))"
      echo "creating $d on $branch (build cache starts cold)" >&2
      git worktree add -b "$branch" "$d" main >&2
    fi
    printf '%s %s\n' "$slug" "$(date -Iseconds)" > "$d/.task"
    printf '%s\n' "$d"
    ;;
  release)
    [ "$#" -eq 1 ] || fail "usage: slot.sh release <slot-path-or-N>"
    slot="$1"
    case "$slot" in
      */*) d="$(realpath -m -- "$slot")" ;;
      [0-9]*) d="$root/.worktrees/slot-$slot" ;;
      *) d="$root/.worktrees/$slot" ;;
    esac
    [[ "${d%/*}" = "$root/.worktrees" && "${d##*/}" =~ ^slot-[1-9][0-9]*$ ]] || fail "not a slot path: $slot"
    [ -f "$d/.task" ] || fail "$d has no .task marker"
    exec 9>"$root/.worktrees/.claim.lock"
    flock 9
    [ -f "$d/.task" ] || fail "$d has no .task marker"
    check_slot "$d"
    check_integrated "$d"
    git -C "$d" switch --no-overwrite-ignore --detach main >&2
    rm -- "$d/.task"
    echo "released $d"
    ;;
  list)
    total=0
    busy=0
    while IFS= read -r d; do
      total=$((total + 1))
      branch="$(git -C "$d" symbolic-ref --quiet --short HEAD || echo detached)"
      if [ -f "$d/.task" ]; then
        busy=$((busy + 1))
        slug=""; ts=""
        read -r slug ts _ < "$d/.task" || true
        echo "busy: $d  branch=$branch  task=$slug  since=$ts"
      else
        echo "free: $d  branch=$branch"
      fi
    done < <(slot_dirs)
    echo "slots: total=$total  busy=$busy"
    ;;
  *)
    usage >&2
    exit 1
    ;;
esac
