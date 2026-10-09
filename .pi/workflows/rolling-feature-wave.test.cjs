const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
const source = fs.readFileSync(path.join(__dirname, 'rolling-feature-wave.js'), 'utf8')
  .replace('export const meta =', 'const meta =');
const execute = new AsyncFunction('args', 'agent', 'pipeline', 'log', source);
const head = 'a'.repeat(40);
const slices = [1, 2, 3].map(id => ({ id: `P${id}`, root: `owned-${id}`, boundary: `components/p${id}/**` }));
async function run(items, status = 'ready', reviewedHead = head) {
  const calls = [];
  const agent = async (_prompt, options) => {
    calls.push(options.label);
    return options.agentType === 'worker'
      ? { status, head }
      : { head: reviewedHead, verdict: 'no-concrete-blocker' };
  };
  const pipeline = (items, ...stages) => Promise.all(items.map(async (item, index) => {
    let result = item;
    for (const stage of stages) result = await stage(result, item, index);
    return result;
  }));
  return { result: await execute({ slices: items }, agent, pipeline, () => {}), calls };
}
(async () => {
  await assert.rejects(run([]), /one to three/);
  await assert.rejects(run([...slices, { ...slices[0], root: 'fourth' }]), /one to three/);
  await assert.rejects(run([slices[0], slices[0]]), /distinct owned checkouts/);
  const ready = await run(slices);
  assert.equal(ready.calls.length, 6);
  assert.equal(ready.result.results.length, 3);
  assert.equal(ready.result.integration_performed, false);
  assert.equal(ready.result.acceptance_updated, false);
  assert.equal(ready.result.production_enablement, false);
  const blocked = await run(slices, 'blocked');
  assert.equal(blocked.calls.length, 3);
  assert.ok(blocked.result.results.every(r => r.review === null));
  const stale = await run([slices[0]], 'ready', 'b'.repeat(40));
  assert.equal(stale.result.results[0].review, null);
  console.log('Workflow capacity, checkout ownership, blocked candidates, reviewed head and supervisor-only promotion checks passed.');
})().catch(error => { console.error(error); process.exitCode = 1; });
