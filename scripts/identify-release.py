#!/usr/bin/env python3
"""Identify a release-plz merge with successful CI, without publishing anything."""

import json
import os
import re
import subprocess
import sys


BOT_ID = 236402640
SHA = re.compile(r"[0-9a-f]{40}\Z")


def api(path, paginate=False):
    command = ["gh", "api"]
    if paginate:
        command += ["--paginate", "--slurp"]
    command.append(path)
    result = json.loads(subprocess.check_output(command, text=True))
    return [item for page in result for item in page] if paginate else result


def release_pr(prs, sha, repository):
    # release-plz itself picks the first associated PR with this branch prefix.
    release_branches = [
        pr for pr in prs if pr.get("head", {}).get("ref", "").startswith("release-plz-")
    ]
    matches = [
        pr for pr in release_branches
        if pr.get("state") == "closed"
        and pr.get("merged_at")
        and pr.get("merge_commit_sha") == sha
        and pr.get("user", {}).get("id") == BOT_ID
        and pr.get("base", {}).get("ref") == "main"
        and (pr.get("base", {}).get("repo") or {}).get("full_name") == repository
        and (pr.get("head", {}).get("repo") or {}).get("full_name") == repository
    ]
    if matches and (len(matches) != 1 or len(release_branches) != 1):
        raise ValueError("Ambiguous release PR association")
    return matches[0] if matches else None


def check_ci(run, jobs, repository):
    if (
        run.get("path") != ".github/workflows/ci.yaml"
        or run.get("event") != "push"
        or run.get("head_branch") != "main"
        or (run.get("head_repository") or {}).get("full_name") != repository
        or run.get("status") != "completed"
        or run.get("conclusion") != "success"
    ):
        raise ValueError("CI must be a successful main-branch push of CI workflow")
    gates = [job for job in jobs if job.get("name") == "main-gate"]
    if len(gates) != 1 or gates[0].get("conclusion") != "success":
        raise ValueError("The main-gate job did not succeed")


def main():
    repository = os.environ["GITHUB_REPOSITORY"]
    event_name = os.environ["GITHUB_EVENT_NAME"]
    run_id = os.environ["CI_RUN_ID"]
    if not re.fullmatch(r"[1-9][0-9]*", run_id):
        raise ValueError("CI run ID must be a positive integer")
    run = api(f"repos/{repository}/actions/runs/{run_id}")
    sha = run.get("head_sha", "")
    if not SHA.fullmatch(sha):
        raise ValueError("Invalid CI commit SHA")
    if event_name == "workflow_run":
        with open(os.environ["GITHUB_EVENT_PATH"], encoding="utf-8") as file:
            event_run = json.load(file)["workflow_run"]
        if any(event_run.get(key) != run.get(key) for key in ("id", "head_sha", "run_attempt", "conclusion")):
            raise ValueError("CI run changed since this event; use its latest completed run")
    prs = api(f"repos/{repository}/commits/{sha}/pulls?per_page=100", paginate=True)
    pr = release_pr(prs, sha, repository)
    if pr is None:
        if event_name == "workflow_dispatch":
            raise ValueError("CI commit is not an eligible release PR merge")
        print("No eligible release PR for this commit; nothing to publish.")
        with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as output:
            print("eligible=false", file=output)
        return
    attempt = run.get("run_attempt")
    if not isinstance(attempt, int) or attempt < 1:
        raise ValueError("Invalid CI run attempt")
    jobs = api(f"repos/{repository}/actions/runs/{run_id}/attempts/{attempt}/jobs?per_page=100")
    check_ci(run, jobs["jobs"], repository)
    number = pr["number"]
    head = pr["head"]["sha"]
    commits = api(f"repos/{repository}/pulls/{number}/commits?per_page=100", paginate=True)
    if not SHA.fullmatch(head) or not commits or commits[-1].get("sha") != head:
        raise ValueError("Release PR head does not match its last commit")
    with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as output:
        print(f"eligible=true\nsha={sha}\npr={number}\npr_head={head}", file=output)
    print(f"Release PR #{number}; CI passed for {sha}.")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"::error::{error}", file=sys.stderr)
        sys.exit(1)
