import os
from pathlib import Path
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parent.parent


class ReleaseTreeTest(unittest.TestCase):
    def test_rebased_tree_and_stale_or_ancestor_heads(self):
        with tempfile.TemporaryDirectory() as directory:
            def git(*args):
                return subprocess.check_output(
                    ["git", "-C", directory, *args], text=True, stderr=subprocess.DEVNULL
                ).strip()

            def commit(message):
                git("add", "file")
                git("-c", "user.name=Test", "-c", "user.email=test@example.com", "commit", "-m", message)
                return git("rev-parse", "HEAD")

            def check(merge, head):
                return subprocess.run(
                    ["bash", str(ROOT / "scripts/check-release-tree.sh")],
                    cwd=directory,
                    env={**os.environ, "RELEASE_SHA": merge, "PR_HEAD": head},
                    capture_output=True,
                ).returncode

            git("init", "-q")
            file = Path(directory) / "file"
            file.write_text("base")
            base = commit("base")
            file.write_text("release")
            head = commit("release")
            git("switch", "--detach", base)
            file.write_text("release")
            rebased = commit("rebased release")
            self.assertEqual(check(rebased, head), 0)
            self.assertEqual(check(head, head), 0)
            file.write_text("unreviewed change")
            stale = commit("main advanced")
            self.assertNotEqual(check(stale, head), 0)
            git("switch", "--detach", head)
            git("-c", "user.name=Test", "-c", "user.email=test@example.com",
                "commit", "--allow-empty", "-m", "merge")
            same_tree_child = git("rev-parse", "HEAD")
            self.assertNotEqual(check(same_tree_child, head), 0)
