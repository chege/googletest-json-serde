import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location("identify_release", ROOT / "scripts/identify-release.py")
identify = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(identify)

REPO = "chege/googletest-json-serde"
MERGE = "a" * 40
HEAD = "b" * 40


def release_pr():
    return {
        "number": 123,
        "state": "closed",
        "merged_at": "2026-01-01T00:00:00Z",
        "merge_commit_sha": MERGE,
        "user": {"id": identify.BOT_ID},
        "base": {"ref": "main", "repo": {"full_name": REPO}},
        "head": {"ref": "release-plz-test", "sha": HEAD, "repo": {"full_name": REPO}},
    }


def ci_run():
    return {
        "id": 42,
        "path": ".github/workflows/ci.yaml",
        "event": "push",
        "head_branch": "main",
        "head_sha": MERGE,
        "head_repository": {"full_name": REPO},
        "status": "completed",
        "conclusion": "success",
        "run_attempt": 1,
    }


class IdentifyReleaseTest(unittest.TestCase):
    def test_only_exact_bot_release_merge_is_eligible(self):
        self.assertIsNotNone(identify.release_pr([release_pr()], MERGE, REPO))
        for field, value in (
            ("state", "open"),
            ("merged_at", None),
            ("merge_commit_sha", "c" * 40),
            ("user", {"id": 1}),
            ("base", {"ref": "other", "repo": {"full_name": REPO}}),
            ("head", {"ref": "feature", "repo": {"full_name": REPO}}),
            ("head", {"ref": "release-plz-test", "repo": {"full_name": "fork/repo"}}),
        ):
            with self.subTest(field=field, value=value):
                pr = release_pr()
                pr[field] = value
                self.assertIsNone(identify.release_pr([pr], MERGE, REPO))

    def test_ambiguous_release_branches_fail(self):
        other = release_pr()
        other["merge_commit_sha"] = "c" * 40
        with self.assertRaisesRegex(ValueError, "Ambiguous"):
            identify.release_pr([release_pr(), other], MERGE, REPO)

    def test_ci_requires_successful_main_push_and_real_gate(self):
        jobs = [{"name": "main-gate", "conclusion": "success"}]
        identify.check_ci(ci_run(), jobs, REPO)
        for field, value in (
            ("path", ".github/workflows/other.yaml"),
            ("event", "pull_request"),
            ("head_branch", "feature"),
            ("head_repository", {"full_name": "fork/repo"}),
            ("status", "in_progress"),
            ("conclusion", "failure"),
        ):
            with self.subTest(field=field):
                run = ci_run()
                run[field] = value
                with self.assertRaises(ValueError):
                    identify.check_ci(run, jobs, REPO)
        for bad_jobs in ([], [{"name": "main-gate", "conclusion": "skipped"}], jobs * 2):
            with self.subTest(jobs=bad_jobs), self.assertRaises(ValueError):
                identify.check_ci(ci_run(), bad_jobs, REPO)

    def run_main(self, run=None, pr=None, event_name="workflow_run", event_run=None, commits=None):
        run = ci_run() if run is None else run
        pr = release_pr() if pr is None else pr
        with tempfile.TemporaryDirectory() as directory:
            event_path = Path(directory) / "event.json"
            output_path = Path(directory) / "output"
            event_path.write_text(json.dumps({"workflow_run": event_run or run}))
            env = {
                "GITHUB_REPOSITORY": REPO,
                "GITHUB_EVENT_NAME": event_name,
                "GITHUB_EVENT_PATH": str(event_path),
                "GITHUB_OUTPUT": str(output_path),
                "CI_RUN_ID": "42",
            }

            def fake_api(path, paginate=False):
                if "/attempts/" in path:
                    return {"jobs": [{"name": "main-gate", "conclusion": "success"}]}
                if "/commits/" in path:
                    return [pr] if pr else []
                if "/pulls/" in path:
                    return commits if commits is not None else [{"sha": HEAD}]
                return run

            with patch.dict(os.environ, env), patch.object(identify, "api", side_effect=fake_api):
                identify.main()
            return output_path.read_text()

    def test_success_and_manual_retry_keep_exact_sha(self):
        for event in ("workflow_run", "workflow_dispatch"):
            with self.subTest(event=event):
                self.assertEqual(
                    self.run_main(event_name=event),
                    f"eligible=true\nsha={MERGE}\npr=123\npr_head={HEAD}\n",
                )

    def test_nonrelease_is_skipped_but_manual_retry_fails(self):
        self.assertEqual(self.run_main(pr={}), "eligible=false\n")
        with self.assertRaisesRegex(ValueError, "not an eligible"):
            self.run_main(pr={}, event_name="workflow_dispatch")

    def test_failed_or_changed_ci_and_changed_pr_head_fail(self):
        failed = ci_run()
        failed["conclusion"] = "failure"
        with self.assertRaisesRegex(ValueError, "successful"):
            self.run_main(run=failed)
        previous = ci_run()
        previous["run_attempt"] = 2
        with self.assertRaisesRegex(ValueError, "changed"):
            self.run_main(event_run=previous)
        with self.assertRaisesRegex(ValueError, "last commit"):
            self.run_main(commits=[{"sha": "c" * 40}])


if __name__ == "__main__":
    unittest.main()
