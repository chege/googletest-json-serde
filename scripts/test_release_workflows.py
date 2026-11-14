from pathlib import Path
import unittest


WORKFLOWS = Path(__file__).resolve().parent.parent / ".github/workflows"


class ReleaseWorkflowsTest(unittest.TestCase):
    def test_trusted_publishing_uses_dispatch_not_workflow_run(self):
        release = (WORKFLOWS / "release.yml").read_text()
        triggers = release.split("\npermissions:", 1)[0]
        self.assertIn("  workflow_dispatch:", triggers)
        self.assertNotIn("  workflow_run:", triggers)
        self.assertIn("id-token: write", release)
        self.assertIn("CI_RUN_ID: ${{ inputs.ci_run_id }}", release)

    def test_ci_dispatches_only_eligible_release(self):
        dispatch = (WORKFLOWS / "release-dispatch.yml").read_text()
        self.assertIn("  workflow_run:", dispatch)
        self.assertIn("actions: write", dispatch)
        self.assertIn("python3 scripts/identify-release.py", dispatch)
        self.assertIn("steps.identify.outputs.eligible == 'true'", dispatch)
        self.assertIn('gh workflow run release.yml --repo "$GITHUB_REPOSITORY" --ref main '
                      '-f "ci_run_id=$CI_RUN_ID"', dispatch)


if __name__ == "__main__":
    unittest.main()
