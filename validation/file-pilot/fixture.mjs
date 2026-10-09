import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash, randomUUID } from 'node:crypto';
import { deflateSync, inflateSync } from 'node:zlib';
import { spawnSync } from 'node:child_process';

const script = fileURLToPath(import.meta.url);
const repo = path.resolve(path.dirname(script), '../..');
const base = path.join(repo, 'target', 'visual');
const owner = 'v001-file-pilot-26cbf184-176d-4a38-b08d-2cd84a920c20';
const root = path.join(base, owner);
const epoch = Date.parse('2025-01-02T12:00:00Z');
const json = value => Buffer.from(JSON.stringify(value, null, 2) + '\n', 'utf8');
const sha = bytes => createHash('sha256').update(bytes).digest('hex');

function plainDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `Not an ordinary directory: ${directory}`);
}
function bounded(directory) {
  assert.equal(path.dirname(directory), base, 'Only fixed generated children of target/visual are allowed');
  const name = path.basename(directory);
  assert.ok(name === owner || new RegExp(`^${owner}-test-[0-9a-f-]{36}$`).test(name), 'Not this generator namespace');
  // Reject redirected ancestors before any fixture read/write. No recursive mkdir across a junction.
  for (let parent = repo; ; parent = path.dirname(parent)) {
    plainDirectory(parent);
    if (path.dirname(parent) === parent) break;
  }
  for (const parent of [path.join(repo, 'target'), base]) {
    if (fs.existsSync(parent)) plainDirectory(parent);
    else assert.throws(() => fs.lstatSync(parent), { code: 'ENOENT' });
  }
}
function marker(directory) {
  return json({ version: 1, owner, directory: path.basename(directory) });
}
function owned(directory) {
  bounded(directory);
  plainDirectory(directory);
  const markerPath = path.join(directory, '.owner.json');
  assert.ok(fs.lstatSync(markerPath).isFile() && !fs.lstatSync(markerPath).isSymbolicLink(), 'Unsafe marker');
  assert.deepEqual(fs.readFileSync(markerPath), marker(directory), 'Missing or mismatched ownership marker');
}
function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit++) crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
  }
  return (crc ^ 0xffffffff) >>> 0;
}
function png() {
  const chunk = (type, data) => {
    const body = Buffer.concat([Buffer.from(type), data]);
    const length = Buffer.alloc(4); length.writeUInt32BE(data.length);
    const crc = Buffer.alloc(4); crc.writeUInt32BE(crc32(body));
    return Buffer.concat([length, body, crc]);
  };
  const header = Buffer.alloc(13);
  header.writeUInt32BE(64, 0); header.writeUInt32BE(48, 4); header[8] = 8; header[9] = 2;
  const rows = Buffer.alloc(48 * (1 + 64 * 3));
  for (let y = 0; y < 48; y++) for (let x = 0; x < 64; x++) {
    const color = ((x >> 3) + (y >> 3)) % 2 ? [89, 215, 235] : [18, 93, 111];
    rows.set(color, y * 193 + 1 + x * 3);
  }
  return Buffer.concat([Buffer.from('89504e470d0a1a0a', 'hex'), chunk('IHDR', header), chunk('IDAT', deflateSync(rows)), chunk('IEND', Buffer.alloc(0))]);
}
function pdf() {
  const stream = number => {
    const text = `BT /F1 24 Tf 36 220 Td (Owned visual fixture page ${number}) Tj ET\n`;
    return `<< /Length ${Buffer.byteLength(text)} >>\nstream\n${text}endstream`;
  };
  const objects = [
    '<< /Type /Catalog /Pages 2 0 R >>',
    '<< /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 >>',
    ...[5, 6].map(content => `<< /Type /Page /Parent 2 0 R /MediaBox [0 0 360 280] /Resources << /Font << /F1 7 0 R >> >> /Contents ${content} 0 R >>`),
    stream(1), stream(2), '<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
  ];
  let text = '%PDF-1.4\n';
  const offsets = [0];
  objects.forEach((object, index) => {
    offsets.push(Buffer.byteLength(text)); text += `${index + 1} 0 obj\n${object}\nendobj\n`;
  });
  const start = Buffer.byteLength(text);
  text += `xref\n0 8\n0000000000 65535 f \n${offsets.slice(1).map(offset => String(offset).padStart(10, '0') + ' 00000 n \n').join('')}trailer\n<< /Size 8 /Root 1 0 R >>\nstartxref\n${start}\n%%EOF\n`;
  return Buffer.from(text, 'ascii');
}
function contents() {
  const files = new Map([
    ['listing/Projects/readme.txt', Buffer.from('Owned nested project fixture.\n')],
    ['listing/Reference/notes.md', Buffer.from('# Owned reference\n')],
    ['listing/Exports/sample.csv', Buffer.from('name,value\nalpha,1\nbeta,2\n')],
    ['listing/readme.txt', Buffer.from('Visual comparison fixture, not personal data.\n')],
    ['listing/zero.txt', Buffer.alloc(0)],
    ['listing/size-1024.bin', Buffer.alloc(1024, 65)],
    ['listing/size-65536.bin', Buffer.alloc(65536, 66)],
    ['listing/café-東京-𐐀.txt', Buffer.from('Unicode fixture: café 東京 𐐀\n')],
    [`listing/${'long-name-'.repeat(12)}final.txt`, Buffer.from('Complete name must survive visual truncation.\n')],
    ['listing/capture.png', png()],
    ['listing/research-notes.pdf', pdf()],
  ]);
  const directories = ['listing', 'listing/Projects', 'listing/Reference', 'listing/Exports', 'listing/Empty'];
  const entries = [...directories.map(name => ({ path: name, kind: 'directory', bytes: null, modifiedUtc: new Date(epoch).toISOString() })),
    ...[...files].map(([name, bytes], index) => ({ path: name, kind: 'file', bytes: bytes.length, sha256: sha(bytes), modifiedUtc: new Date(epoch + index * 86400000).toISOString() }))];
  return { files, directories, entries };
}
function workspace(directory) {
  const native = value => process.platform === 'win32'
    ? { encoding: 'Windows', units: Array.from({ length: value.length }, (_, i) => value.charCodeAt(i)) }
    : { encoding: 'Unix', units: [...Buffer.from(value)] };
  const listing = path.join(directory, 'listing');
  return json({ version: 1, geometry: { position: null, size: [1200, 800], maximized: false },
    panes: [{ tabs: [native(listing)], active: 0 }], focused: 0, layout: { kind: 'Pane', pane: 0 },
    bookmarks: ['Projects', 'Reference', 'Exports'].map(name => native(path.join(listing, name))), recents: [native(listing)] });
}
function manifest(directory) {
  return json({ version: 1, owner, root: directory, listing: path.join(directory, 'listing'),
    settings: path.join(directory, 'workspace.json'), generatorSha256: sha(fs.readFileSync(script)),
    requestedLogicalClient: [1200, 800], actualClientPixels: null, actualDpi: null,
    theme: 'Mado currently dark only; match reference manually', entries: contents().entries });
}
function create(directory) {
  bounded(directory);
  for (const parent of [path.join(repo, 'target'), base]) {
    if (!fs.existsSync(parent)) fs.mkdirSync(parent);
    plainDirectory(parent);
  }
  fs.mkdirSync(directory); // Exclusive claim. Never overwrite an existing fixture.
  fs.writeFileSync(path.join(directory, '.owner.json'), marker(directory), { flag: 'wx' });
  const fixture = contents();
  for (const name of fixture.directories) fs.mkdirSync(path.join(directory, name));
  for (const [name, bytes] of fixture.files) fs.writeFileSync(path.join(directory, name), bytes, { flag: 'wx' });
  for (const entry of [...fixture.entries].reverse()) {
    const date = new Date(entry.modifiedUtc); fs.utimesSync(path.join(directory, entry.path), date, date);
  }
  fs.writeFileSync(path.join(directory, 'workspace.json'), workspace(directory), { flag: 'wx' });
  fs.writeFileSync(path.join(directory, 'manifest.json'), manifest(directory), { flag: 'wx' });
  validate(directory);
}
function inventory(directory) {
  owned(directory);
  const expected = new Map([
    ['.owner.json', 'file'], ['manifest.json', 'file'], ['workspace.json', 'file'],
    ...contents().entries.map(entry => [entry.path, entry.kind]),
  ]);
  const seen = [];
  function walk(relative) {
    for (const name of fs.readdirSync(path.join(directory, relative))) {
      const item = relative ? `${relative}/${name}` : name;
      const stat = fs.lstatSync(path.join(directory, item));
      assert.ok(!stat.isSymbolicLink(), `Refuse link/reparse redirection: ${item}`);
      assert.equal(expected.get(item), stat.isDirectory() ? 'directory' : stat.isFile() ? 'file' : 'other', `Unknown or changed output: ${item}`);
      seen.push(item);
      if (stat.isDirectory()) walk(item);
    }
  }
  walk('');
  assert.equal(seen.length, expected.size, 'Missing generated output');
  return seen;
}
function validate(directory) {
  inventory(directory);
  assert.deepEqual(fs.readFileSync(path.join(directory, 'manifest.json')), manifest(directory), 'Changed manifest or generator version');
  for (const entry of contents().entries) {
    const file = path.join(directory, entry.path);
    assert.equal(fs.statSync(file).mtimeMs, Date.parse(entry.modifiedUtc), `Changed timestamp: ${entry.path}`);
    if (entry.kind === 'file') {
      const bytes = fs.readFileSync(file);
      assert.equal(bytes.length, entry.bytes); assert.equal(sha(bytes), entry.sha256);
    }
  }
  // Native saves may change workspace state, but a later launch must not restore personal locations.
  const settings = JSON.parse(fs.readFileSync(path.join(directory, 'workspace.json'), 'utf8'));
  assert.equal(settings.version, 1);
  assert.ok(settings.panes.length > 0);
  const locations = [...settings.panes.flatMap(pane => { assert.ok(pane.tabs.length > 0); return pane.tabs; }), ...settings.bookmarks, ...settings.recents];
  for (const location of locations) {
    const windows = process.platform === 'win32';
    assert.equal(location.encoding, windows ? 'Windows' : 'Unix');
    assert.ok(Array.isArray(location.units) && location.units.length > 0 && location.units.length <= 32766);
    assert.ok(location.units.every(unit => Number.isInteger(unit) && unit > 0 && unit <= (windows ? 65535 : 255)));
    const value = windows ? String.fromCharCode(...location.units) : Buffer.from(location.units).toString('utf8');
    const relative = path.relative(path.join(directory, 'listing'), value);
    assert.ok(path.isAbsolute(value) && !path.isAbsolute(relative) && relative !== '..' && !relative.startsWith(`..${path.sep}`), 'Workspace location escaped fixture listing');
  }
}
function clean(directory) {
  validate(directory); // Refuse replaced/edited fixture content as well as unknown paths before deleting anything.
  const seen = inventory(directory);
  // ponytail: trusted single-operator fixtures only. Close apps first; this is not an adversarial concurrent-writer sandbox.
  for (const item of seen.sort((a, b) => b.split('/').length - a.split('/').length)) {
    const file = path.join(directory, item);
    if (fs.lstatSync(file).isDirectory()) fs.rmdirSync(file); else fs.unlinkSync(file);
  }
  fs.rmdirSync(directory);
}
function test() {
  const a = path.join(base, `${owner}-test-${randomUUID()}`);
  const b = path.join(base, `${owner}-test-${randomUUID()}`);
  create(a); create(b);
  const read = directory => JSON.parse(fs.readFileSync(path.join(directory, 'manifest.json'), 'utf8'));
  assert.deepEqual(read(a).entries, read(b).entries);
  for (const [name, bytes] of contents().files) assert.deepEqual(fs.readFileSync(path.join(a, name)), bytes);
  const settings = JSON.parse(workspace(a));
  const units = settings.panes[0].tabs[0].units;
  assert.equal(process.platform === 'win32' ? String.fromCharCode(...units) : Buffer.from(units).toString(), path.join(a, 'listing'));
  const workspaceFile = path.join(a, 'workspace.json');
  fs.writeFileSync(workspaceFile, workspace(b));
  assert.throws(() => validate(a), /Workspace location escaped/);
  fs.writeFileSync(workspaceFile, workspace(a));
  assert.throws(() => create(a), { code: 'EEXIST' });
  assert.throws(() => clean(repo));
  const sentinel = path.join(b, 'listing/readme.txt');
  const saved = fs.readFileSync(sentinel);
  const changed = path.join(a, 'listing/readme.txt');
  const changedTime = new Date(contents().entries.find(entry => entry.path === 'listing/readme.txt').modifiedUtc);
  fs.writeFileSync(changed, 'owned test replacement'); fs.utimesSync(changed, changedTime, changedTime);
  assert.throws(() => validate(a)); assert.throws(() => clean(a));
  fs.writeFileSync(changed, saved); fs.utimesSync(changed, changedTime, changedTime);
  const child = spawnSync(process.execPath, [script, 'clean', b], { encoding: 'utf8' });
  assert.notEqual(child.status, 0); assert.deepEqual(fs.readFileSync(sentinel), saved);
  const markerFile = path.join(a, '.owner.json');
  fs.writeFileSync(markerFile, '{}'); assert.throws(() => clean(a)); fs.writeFileSync(markerFile, marker(a));
  const manifestFile = path.join(a, 'manifest.json');
  fs.writeFileSync(manifestFile, json({ entries: [{ path: sentinel }] }));
  assert.throws(() => clean(a)); assert.deepEqual(fs.readFileSync(sentinel), saved);
  fs.writeFileSync(manifestFile, manifest(a));
  const extra = path.join(a, 'not-generated.txt');
  fs.writeFileSync(extra, 'owned test only'); assert.throws(() => clean(a)); fs.unlinkSync(extra);
  const link = path.join(a, 'listing/Empty');
  fs.rmdirSync(link); fs.symlinkSync(path.join(b, 'listing'), link, process.platform === 'win32' ? 'junction' : 'dir');
  assert.throws(() => clean(a)); assert.throws(() => validate(a));
  assert.deepEqual(fs.readFileSync(sentinel), saved);
  fs.unlinkSync(link); fs.mkdirSync(link); fs.utimesSync(link, new Date(epoch), new Date(epoch));
  fs.utimesSync(path.join(a, 'listing'), new Date(epoch), new Date(epoch));
  const image = png();
  assert.equal(image.readUInt32BE(16), 64); assert.equal(image.readUInt32BE(20), 48);
  let offset = 8;
  while (offset < image.length) {
    const length = image.readUInt32BE(offset);
    const body = image.subarray(offset + 4, offset + 8 + length);
    assert.equal(crc32(body), image.readUInt32BE(offset + 8 + length));
    if (body.subarray(0, 4).toString() === 'IDAT') assert.equal(inflateSync(body.subarray(4)).length, 48 * 193);
    offset += length + 12;
  }
  const document = pdf().toString('ascii');
  const xref = Number(/startxref\n(\d+)/.exec(document)[1]);
  assert.equal(document.slice(xref, xref + 4), 'xref');
  const offsets = document.slice(xref).split('\n').slice(3, 10);
  offsets.forEach((line, i) => assert.ok(document.slice(Number(line.slice(0, 10))).startsWith(`${i + 1} 0 obj\n`)));
  assert.ok(document.includes('/Count 2')); // Structural check, not a renderer or GUI preview claim.
  validate(a); validate(b); clean(a); clean(b);
  console.log('PASS: deterministic bytes/times/manifest, native settings path, exclusive claim, marker/manifest/extra/link refusal, external-path CLI rejection, PNG chunks and PDF offsets. Owned test roots removed.');
}

try {
  assert.equal(process.argv.length, 3, 'Use exactly run, validate, test or clean. No operator path accepted.');
  switch (process.argv[2]) {
    case 'run': create(root); console.log(manifest(root).toString()); break;
    case 'validate': validate(root); console.log(`PASS: ${root}`); break;
    case 'test': test(); break;
    case 'clean': clean(root); console.log(`Removed only ${root}`); break;
    default: throw new Error('Use run, validate, test or clean');
  }
} catch (error) {
  console.error(error.message); process.exitCode = 1;
}
