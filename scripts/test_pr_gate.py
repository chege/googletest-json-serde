import json
import os
from pathlib import Path
import subprocess
import unittest


ROOT = Path(__file__).resolve().parent.parent


class PrGateTest(unittest.TestCase):
    def run_gate(self, needs):
        return subprocess.run(
            ["bash", str(ROOT / "scripts/check-pr-gate.sh")],
            env={**os.environ, "NEEDS_JSON": json.dumps(needs)},
            capture_output=True,
            text=True,
        )

    def test_all_required_jobs_must_succeed(self):
        self.assertEqual(
            self.run_gate({"first": {"result": "success"}, "second": {"result": "success"}}).returncode,
            0,
        )
        for result in ("failure", "cancelled", "skipped", None):
            with self.subTest(result=result):
                self.assertNotEqual(
                    self.run_gate({"first": {"result": "success"}, "second": {"result": result}}).returncode,
                    0,
                )

    def test_empty_or_missing_results_fail(self):
        for needs in ({}, {"first": {}}, None):
            with self.subTest(needs=needs):
                self.assertNotEqual(self.run_gate(needs).returncode, 0)


if __name__ == "__main__":
    unittest.main()
