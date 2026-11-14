# How to Contribute

We’re excited to welcome your patches and contributions! Just a handful of tiny guidelines stand between you and
everlasting fame.

## Commit Message Guidelines

This project follows the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

Format: `<type>(optional scope): <description>`

### Examples

- `feat: add support for new matcher`
- `fix(cli): handle invalid input gracefully`
- `docs: update README with usage examples`
- `test: improve matcher coverage`

### Allowed types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc)
- `refactor`: Code refactoring without behavior change
- `perf`: Performance improvements
- `test`: Adding or fixing tests
- `build`: Build system or dependencies
- `ci`: CI-related changes
- `chore`: Maintenance
- `revert`: Revert a commit

## Releasing

Release PRs are requested manually; publishing is automatic after they are
merged and the merged commit passes CI. Rebase-merge pull requests so their
individual conventional commits are available for the changelog.

Configure the `release` environment to allow deployments from `main` only.
After the `pr-gate` check is available on `main`, make it required in the
default-branch ruleset and enable that ruleset.

1. Run **Create or Update Release PR** from `main`.
2. Review the proposed version and changelog against all changes since the last
   published release. Check public API and behavior changes, and include
   migration guidance for breaking changes. Review the packaged crate as well
   as the generated notes.
3. If `main` has changed since the release PR was prepared, run **Create or
   Update Release PR** again and review the updated version and changelog.
   Rebase-merge the release PR after its checks pass. Closing it without merging
   does not publish.
4. After CI passes on the merged commit, **Dispatch Release After CI** starts
   **Publish Crate and Create GitHub Release** for that exact commit. Other PRs
   do not trigger publication. Confirm the workflows, published crate, tag, and
   GitHub release agree. If the merged tree differs from the reviewed PR,
   publishing fails rather than releasing unreviewed changes.
5. Update the `sanity` crate dependency, lockfile, and tests for the published
   version, then run `cargo test` from `sanity/`.

If publication only partly succeeds, first inspect crates.io, the git tags, and
the GitHub release to determine what was published. Do not blindly rerun the
workflow: published crate versions cannot be overwritten. Repair missing
artifacts where appropriate or prepare a new release; yank a bad version only
after deciding it should no longer be used.

To retry a failed publication after checking its state, rerun the publishing
workflow or manually run **Publish Crate and Create GitHub Release** from `main`
with the ID of the successful `main` CI run for the release commit. If dispatch
failed before starting publication, rerun **Dispatch Release After CI**. A
failed main CI run must be rerun successfully, or a new release PR prepared
after fixing CI. Release-plz may skip a version whose tag already exists, so a
rerun does not necessarily repair a partial release.
