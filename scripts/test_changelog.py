import json
from pathlib import Path
import subprocess
import tempfile
import tomllib
import unittest


ROOT = Path(__file__).resolve().parent.parent


class ChangelogTest(unittest.TestCase):
    def test_breaking_descriptions_are_rendered_for_scoped_and_unscoped_commits(self):
        config = tomllib.loads((ROOT / "release-plz.toml").read_text())
        body = config["changelog"]["body"]
        body = body[body.index("{% for group") :]

        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            subprocess.run(["git", "init", "-q", str(directory)], check=True)
            subprocess.run(
                ["git", "-c", "user.name=Test", "-c", "user.email=test@example.com",
                 "commit", "--allow-empty", "-qm", "chore: base"],
                cwd=directory,
                check=True,
            )
            subprocess.run(["git", "tag", "v0.5.0"], cwd=directory, check=True)
            cliff_config = directory / "cliff.toml"
            cliff_config.write_text(
                "[changelog]\ntrim = "
                + str(config["changelog"]["trim"]).lower()
                + "\nbody = "
                + json.dumps(body)
                + '\n[git]\nconventional_commits = true\n'
                + 'commit_parsers = [{ message = "^fix", group = "Fixes" }]\n'
            )
            context = subprocess.run(
                ["git-cliff", "--offline", "--config", str(cliff_config), "--unreleased", "--context",
                 "--with-commit", "fix(path)!: reject indices\n\nBREAKING CHANGE: use canonical indices",
                 "--with-commit", "fix!: change arrays\n\nBREAKING CHANGE: remove inner brackets",
                 "--with-commit", "fix!: change api\n\nBREAKING CHANGE: Update calls.\n- Use foo\n- Use bar",
                 "--with-commit", "fix: ordinary correction"],
                cwd=directory,
                check=True,
                capture_output=True,
                text=True,
            )
            releases = json.loads(context.stdout)
            for commit in releases[0]["commits"]:
                if commit["message"] == "reject indices":
                    commit["links"] = [{"text": "#123", "href": "https://example.com/123"}]
            context_file = directory / "context.json"
            context_file.write_text(json.dumps(releases))
            output = subprocess.run(
                ["git-cliff", "--offline", "--config", str(cliff_config), "--from-context", str(context_file)],
                cwd=directory,
                check=True,
                capture_output=True,
                text=True,
            ).stdout

        self.assertIn(
            "- *(path)* [**breaking**] reject indices ([#123](https://example.com/123))"
            " - **Breaking change:** use canonical indices",
            output,
        )
        self.assertIn("- [**breaking**] change arrays - **Breaking change:** remove inner brackets", output)
        self.assertIn(
            "- [**breaking**] change api - **Breaking change:** Update calls.\n  - Use foo\n  - Use bar",
            output,
        )
        self.assertIn("- ordinary correction", output)
        self.assertEqual(output.count("**Breaking change:**"), 3)


if __name__ == "__main__":
    unittest.main()
