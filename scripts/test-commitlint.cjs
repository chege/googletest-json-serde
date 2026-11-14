const assert = require('node:assert/strict');
const { spawnSync } = require('node:child_process');
const test = require('node:test');

function lint(message) {
  const result = spawnSync('npx', ['--no', '--', 'commitlint'], {
    input: `${message}\n`,
    encoding: 'utf8',
  });
  if (result.error) throw result.error;
  return result.status;
}

test('accepts conventional commits, including release commits', () => {
  for (const message of ['fix: correct behavior', 'fix(api)!: change behavior', 'chore: release']) {
    assert.equal(lint(message), 0, message);
  }
});

test('rejects non-conventional messages normally ignored by commitlint', () => {
  for (const message of ['fixup! add feature', 'v1.2.3', 'Merge branch main', 'Revert "change"']) {
    assert.notEqual(lint(message), 0, message);
  }
});
