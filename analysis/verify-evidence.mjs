import assert from 'node:assert/strict';
import { readFile, access } from 'node:fs/promises';
import { createHash } from 'node:crypto';

const expected = '07d106ca23adab9115af3d529b6fc4bb27dd831e0923103b655707e76326363e';
const binary = await readFile('FPilot.exe');
assert.equal(createHash('sha256').update(binary).digest('hex'), expected, 'Binary identity changed');
assert.equal(binary.length, 2184784);
const index = JSON.parse(await readFile('analysis/evidence-index.json', 'utf8'));
const spec = await readFile('SPECIFICATION.md', 'utf8');
const cited = new Set(spec.match(/\bE-[a-z]+(?:-[a-z]+)*\b/g));
for (const alias of cited) assert(index[alias], `Unknown evidence citation: ${alias}`);
for (const [alias, entry] of Object.entries(index)) {
  const record = JSON.parse(await readFile(entry.file, 'utf8'));
  assert.equal(record.evidence_id, entry.evidence_id, alias);
  assert.equal(record.operation, entry.operation, alias);
  assert.equal(record.subject?.digest?.sha256, expected, `${alias}: wrong artifact`);
  assert(Object.hasOwn(record, 'normalized_result'), `${alias}: missing complete record`);
}
for (const link of spec.matchAll(/\]\(([^)]+)\)/g)) {
  assert(/^(?:[A-Z]:\/|https?:\/\/)/i.test(link[1]), `Relative link: ${link[1]}`);
  if (!link[1].startsWith('http')) await access(link[1]);
}
for (const [alias, file] of [
  ['E-shader-fragment', 'analysis/rea/filepilot-shape.frag.glsl'],
  ['E-shader-vertex', 'analysis/rea/filepilot-shape.vert.glsl'],
]) {
  const record = JSON.parse(await readFile(index[alias].file, 'utf8'));
  const bytes = Buffer.from(record.normalized_result.bytes_hex, 'hex');
  assert.equal(record.normalized_result.complete, true, `${alias}: partial read`);
  assert.equal(bytes.at(-1), 0, `${alias}: missing terminator`);
  assert.deepEqual(await readFile(file), bytes.subarray(0, -1), `${alias}: extracted text changed`);
}
console.log(`Verified original binary, ${Object.keys(index).length} indexed records, ${cited.size} cited aliases, specification links, and exact extracted shader bytes.`);
console.log('This checks evidence integrity, not application behavior.');
