'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const taskfile = fs.readFileSync(path.resolve(__dirname, '../Taskfile.yml'));
const text = taskfile.toString('utf8').replace(/\r\n/g, '\n');
// Only run relative Cargo cleanup here. Refuse Taskfile cwd overrides or other commands.
assert.ok(!/^(?:dir|includes|env|vars):/m.test(text), 'Unexpected global Taskfile configuration');
assert.match(text, /\n  clean:\n    desc: [^\n]+\n    cmds:\n(?:      - cargo clean --target-dir target(?: --profile (?:dev|release)| --doc)?(?: --offline)?\n?)+$/);

const temp = fs.realpathSync(os.tmpdir());
const fixture = fs.mkdtempSync(path.join(temp, 'mado-s020-clean-'));
const ownerPath = path.join(fixture, '.s020-owner');
const owner = Buffer.from(`S020 owned fixture: ${fixture}\n`);
const identity = fs.lstatSync(fixture);
let childFinished = true;
let ownerEstablished = false;

function assertOwned() {
  assert.ok(path.isAbsolute(fixture));
  assert.equal(path.dirname(fixture), temp);
  assert.ok(path.basename(fixture).startsWith('mado-s020-clean-'));
  assert.equal(fs.realpathSync(fixture), fixture);
  const current = fs.lstatSync(fixture);
  assert.ok(current.isDirectory() && !current.isSymbolicLink());
  assert.equal(current.dev, identity.dev);
  assert.equal(current.ino, identity.ino);
  assert.ok(!fs.lstatSync(ownerPath).isSymbolicLink());
  assert.deepEqual(fs.readFileSync(ownerPath), owner);
}

function put(relative, bytes) {
  const destination = path.join(fixture, relative);
  assert.ok(destination.startsWith(fixture + path.sep));
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
}

try {
  fs.writeFileSync(ownerPath, owner, { flag: 'wx' });
  ownerEstablished = true;
  const retained = new Map([
    ['Taskfile.yml', taskfile],
    ['Cargo.toml', Buffer.from('[package]\nname = "mado-clean-fixture"\nversion = "0.1.0"\nedition = "2021"\n')],
    ['src/main.rs', Buffer.from('fn main() {}\n')],
    ['design/layout-marker.txt', Buffer.from('Generated design sentinel\n')],
    ['target/p22/fixtures/owned-data/sentinel.bin', Buffer.from([0, 255, 13, 10, 128, 1])],
    ['target/visual/comparison/sentinel.bin', Buffer.from([255, 0, 128, 13, 10, 2])],
    ['target/p22/build/auxiliary-marker.txt', Buffer.from('Not a root app build\n')],
    ['target/other-data/sentinel.txt', Buffer.from('Non-build sibling\n')],
  ]);
  for (const [relative, bytes] of retained) put(relative, bytes);
  const artifacts = [
    'target/debug/mado-clean-fixture.exe',
    'target/debug/deps/fake-build-artifact.rlib',
    'target/release/mado-clean-fixture.exe',
    'target/release/deps/fake-build-artifact.rlib',
    'target/doc/mado_clean_fixture/index.html',
    'target/doc/static.files/fake-doc-artifact.css',
  ];
  for (const relative of artifacts) put(relative, Buffer.from('Generated fake build artifact\n'));

  assertOwned();
  const copiedTaskfile = path.join(fixture, 'Taskfile.yml');
  assert.equal(path.dirname(copiedTaskfile), fixture);
  const env = { ...process.env, CARGO_NET_OFFLINE: 'true', NO_COLOR: '1' };
  delete env.CARGO_BUILD_TARGET;
  delete env.CARGO_TARGET_DIR;
  console.log(`Owned fixture: ${fixture}`);
  console.log(`task --dir "${fixture}" --taskfile "${copiedTaskfile}" clean`);
  childFinished = false;
  const result = spawnSync('task', ['--dir', fixture, '--taskfile', copiedTaskfile, 'clean'], {
    cwd: fixture,
    env,
    encoding: 'utf8',
    windowsHide: true,
  });
  childFinished = !result.error && result.signal === null && Number.isInteger(result.status);
  process.stdout.write(result.stdout || '');
  process.stderr.write(result.stderr || '');
  assert.ifError(result.error);
  assert.equal(result.signal, null, 'Task did not finish normally; retaining fixture');
  assert.equal(result.status, 0, 'Copied Taskfile clean failed');
  assertOwned();
  // Check retention first so the old whole-target clean fails on data loss.
  for (const [relative, bytes] of retained) {
    assert.ok(fs.existsSync(path.join(fixture, relative)), `Retained file missing: ${relative}`);
    assert.deepEqual(fs.readFileSync(path.join(fixture, relative)), bytes, `Retained bytes changed: ${relative}`);
  }
  for (const relative of artifacts) {
    assert.ok(!fs.existsSync(path.join(fixture, relative)), `Build artifact remains: ${relative}`);
  }
  console.log(`PASS: ${artifacts.length} build artifacts removed; ${retained.size} retained files byte-identical`);
} finally {
  if (ownerEstablished && childFinished) {
    try {
      assertOwned();
      // Recursive removal is limited to this create-new fixture after its child has finished.
      fs.rmSync(fixture, { recursive: true });
      console.log(`Removed owned fixture: ${fixture}`);
    } catch (error) {
      console.error(`Retained fixture: ${fixture}; cleanup ownership/check failed: ${error.message}`);
      process.exitCode = 1;
    }
  } else {
    console.error(`Retained fixture: ${fixture}; ownership or child completion uncertain`);
  }
}
