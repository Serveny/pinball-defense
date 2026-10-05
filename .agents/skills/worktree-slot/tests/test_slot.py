import shutil
import subprocess
import tempfile
import unittest
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "slot.sh"


class SlotTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="slot tests ")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "repo with spaces"
        self.root.mkdir()
        self.git("init", "-b", "main")
        self.git("config", "user.name", "Slot Tests")
        self.git("config", "user.email", "slots@example.invalid")
        self.git("config", "commit.gpgsign", "false")
        self.git("config", "core.hooksPath", "/dev/null")
        self.script = self.root / ".agents/skills/worktree-slot/slot.sh"
        self.script.parent.mkdir(parents=True)
        shutil.copy2(SCRIPT, self.script)
        (self.root / ".gitignore").write_text(".worktrees/\n.task\ntarget/\nsaves/\n")
        (self.root / "feature.txt").write_text("initial\n")
        self.git("add", ".")
        self.git("commit", "-m", "initial")

    def git(self, *args, cwd=None):
        return subprocess.run(
            ["git", *args], cwd=cwd or self.root, check=True,
            text=True, capture_output=True,
        ).stdout.strip()

    def run_slot(self, *args, cwd=None):
        return subprocess.run(
            ["bash", str(self.script), *args], cwd=cwd or self.root,
            text=True, capture_output=True,
        )

    def claim(self, slug, branch=None):
        args = ["claim", slug]
        if branch is not None:
            args.append(branch)
        result = self.run_slot(*args)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(result.stdout.splitlines()), 1)
        path = Path(result.stdout.strip())
        self.assertTrue(path.is_absolute())
        self.assertTrue(path.is_dir())
        return path

    def commit_feature(self, slot, text="feature\n"):
        (slot / "feature.txt").write_text(text)
        self.git("add", "feature.txt", cwd=slot)
        self.git("commit", "-m", "feature", cwd=slot)
        return self.git("rev-parse", "HEAD", cwd=slot)

    def test_feature_branch_and_list(self):
        slot = self.claim("microwave-effects")
        self.assertEqual(self.git("branch", "--show-current", cwd=slot), "feat/microwave-effects")
        result = self.run_slot("list")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("branch=feat/microwave-effects", result.stdout)
        self.assertIn("task=microwave-effects", result.stdout)

    def test_custom_branch(self):
        slot = self.claim("microwave-effects", "fix/microwave-effects")
        self.assertEqual(self.git("branch", "--show-current", cwd=slot), "fix/microwave-effects")

    def test_merge_release_reuse_preserves_branch_and_ignored_files(self):
        slot = self.claim("first")
        commit = self.commit_feature(slot)
        cache = slot / "target/cache"
        cache.parent.mkdir()
        cache.write_text("warm")
        save = slot / "saves/local"
        save.parent.mkdir()
        save.write_text("runtime")
        self.git("merge", "--ff-only", "feat/first")
        result = self.run_slot("release", str(slot))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse((slot / ".task").exists())
        self.assertEqual(self.git("branch", "--show-current", cwd=slot), "")
        self.assertEqual(self.git("rev-parse", "feat/first"), commit)
        self.assertEqual(self.claim("second"), slot)
        self.assertEqual(cache.read_text(), "warm")
        self.assertEqual(save.read_text(), "runtime")
        self.assertEqual(self.git("rev-parse", "feat/first"), commit)

    def test_cherry_pick_release(self):
        slot = self.claim("cherry")
        commit = self.commit_feature(slot)
        (self.root / "other.txt").write_text("advance main\n")
        self.git("add", "other.txt")
        self.git("commit", "-m", "other")
        self.git("cherry-pick", commit)
        self.assertNotEqual(self.git("rev-parse", "HEAD"), commit)
        result = self.run_slot("release", "1")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.git("rev-parse", "feat/cherry"), commit)

    def test_missing_marker_does_not_destroy_dirty_work(self):
        slot = self.claim("dirty")
        (slot / ".task").unlink()
        (slot / "feature.txt").write_text("unfinished\n")
        (slot / "untracked.txt").write_text("keep me\n")
        result = self.run_slot("claim", "next")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("uncommitted changes", result.stderr)
        self.assertEqual((slot / "feature.txt").read_text(), "unfinished\n")
        self.assertEqual((slot / "untracked.txt").read_text(), "keep me\n")

    def test_release_preserves_semantic_whitespace_differences(self):
        slot = self.claim("spaces")
        self.commit_feature(slot, 'const MESSAGE: &str = "hello world";\n')
        (self.root / "feature.txt").write_text('const MESSAGE: &str = "helloworld";\n')
        self.git("add", "feature.txt")
        self.git("commit", "-m", "different string")
        result = self.run_slot("release", "1")
        self.assertNotEqual(result.returncode, 0)
        self.assertTrue((slot / ".task").exists())
        self.assertIn('"hello world"', (slot / "feature.txt").read_text())

    def test_empty_commit_requires_merge(self):
        slot = self.claim("empty")
        self.git("commit", "--allow-empty", "-m", "empty", cwd=slot)
        result = self.run_slot("release", "1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("without comparable patches", result.stderr)
        self.assertTrue((slot / ".task").exists())

    def test_binary_asset_cherry_pick_release(self):
        slot = self.claim("asset")
        (slot / "asset.bin").write_bytes(b"asset\0data\xff")
        self.git("add", "asset.bin", cwd=slot)
        self.git("commit", "-m", "binary asset", cwd=slot)
        commit = self.git("rev-parse", "HEAD", cwd=slot)
        (self.root / "other.txt").write_text("advance main\n")
        self.git("add", "other.txt")
        self.git("commit", "-m", "other")
        self.git("cherry-pick", commit)
        result = self.run_slot("release", "1")
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_different_binary_asset_blocks_release(self):
        slot = self.claim("asset")
        (slot / "asset.bin").write_bytes(b"feature\0data\xff")
        self.git("add", "asset.bin", cwd=slot)
        self.git("commit", "-m", "binary asset", cwd=slot)
        (self.root / "asset.bin").write_bytes(b"different\0data\xff")
        self.git("add", "asset.bin")
        self.git("commit", "-m", "different asset")
        result = self.run_slot("release", "1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("not merged or cherry-picked", result.stderr)
        self.assertTrue((slot / ".task").exists())

    def test_missing_marker_does_not_destroy_unmerged_commit(self):
        slot = self.claim("unmerged")
        commit = self.commit_feature(slot)
        (slot / ".task").unlink()
        result = self.run_slot("claim", "next")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("not merged or cherry-picked", result.stderr)
        self.assertEqual(self.git("rev-parse", "HEAD", cwd=slot), commit)
        self.assertEqual(self.git("rev-parse", "feat/unmerged"), commit)

    def test_release_refuses_dirty_and_unmerged_work(self):
        slot = self.claim("pending")
        (slot / "untracked.txt").write_text("keep")
        result = self.run_slot("release", "slot-1")
        self.assertNotEqual(result.returncode, 0)
        self.assertTrue((slot / ".task").exists())
        self.git("add", "untracked.txt", cwd=slot)
        self.git("commit", "-m", "pending", cwd=slot)
        result = self.run_slot("release", "slot-1")
        self.assertNotEqual(result.returncode, 0)
        self.assertTrue((slot / ".task").exists())

    def test_duplicate_and_invalid_branch_do_not_change_slot(self):
        slot = self.claim("unique")
        head = self.git("rev-parse", "HEAD", cwd=slot)
        for args in [("unique",), ("bad slug",), ("valid", "bad..branch"), ("valid", "main")]:
            with self.subTest(args=args):
                result = self.run_slot("claim", *args)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(self.git("rev-parse", "HEAD", cwd=slot), head)
                self.assertEqual(self.git("branch", "--show-current", cwd=slot), "feat/unique")

    def test_old_numbered_slot_can_be_reused(self):
        slot = self.root / ".worktrees/slot-1"
        self.git("worktree", "add", "-b", "slot-1", str(slot), "main")
        self.assertEqual(self.claim("new-feature"), slot)
        self.assertEqual(self.git("branch", "--show-current", cwd=slot), "feat/new-feature")

    def test_concurrent_claims_get_distinct_slots(self):
        with ThreadPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(lambda slug: self.run_slot("claim", slug), ["first", "second"]))
        for result in results:
            self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotEqual(results[0].stdout, results[1].stdout)

    def test_invocation_outside_repo_and_from_slot(self):
        result = self.run_slot("claim", "outside", cwd=self.root.parent)
        self.assertEqual(result.returncode, 0, result.stderr)
        slot = Path(result.stdout.strip())
        result = subprocess.run(
            ["bash", str(slot / ".agents/skills/worktree-slot/slot.sh"), "claim", "nested"],
            cwd=slot / ".agents", text=True, capture_output=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotEqual(Path(result.stdout.strip()), slot)

    def test_release_rejects_non_slot_path(self):
        marker = self.root / ".task"
        marker.write_text("protect main")
        result = self.run_slot("release", str(self.root))
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(marker.read_text(), "protect main")

    def test_unmerged_merge_commit_blocks_cherry_equivalence(self):
        slot = self.claim("merge")
        self.git("switch", "-c", "side", cwd=slot)
        commit = self.commit_feature(slot)
        self.git("switch", "feat/merge", cwd=slot)
        self.git("merge", "--no-ff", "-m", "merge side", "side", cwd=slot)
        self.git("cherry-pick", commit)
        result = self.run_slot("release", "1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("merge commits outside main", result.stderr)
        self.assertTrue((slot / ".task").exists())


if __name__ == "__main__":
    unittest.main()
