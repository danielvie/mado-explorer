import { createRequire } from 'node:module';
import { readFile, writeFile, mkdir, access } from 'node:fs/promises';
import { resolve, basename } from 'node:path';
import assert from 'node:assert/strict';

const allowed = new Set(['binary_overview', 'search_strings', 'xrefs', 'resolve_containing_procedure', 'analyze_function', 'procedure_pseudo_code', 'procedure_info', 'batch_decompile', 'read_bytes', 'search_procedures', 'find_xrefs_to_name', 'procedure_callers', 'procedure_assembly', 'inspect_native_instruction']);
function validate(requests) {
  assert(Array.isArray(requests) && requests.length > 0);
  for (const q of requests) {
    assert(allowed.has(q.name), `Not a static analysis query: ${q.name}`);
    assert(/^[a-z0-9-]+$/.test(q.label), 'Invalid output label');
    assert(q.arguments && typeof q.arguments === 'object');
  }
  assert.equal(new Set(requests.map(q => q.label)).size, requests.length);
}
if (process.argv[2] === '--check') {
  validate([{ name: 'xrefs', label: 'example', arguments: { address: '0x140001000' } }]);
  assert.throws(() => validate([{ name: 'capture_process_scenario', label: 'example', arguments: {} }]));
  assert.throws(() => validate([{ name: 'xrefs', label: '../escape', arguments: {} }]));
  console.log('REA batch input checks passed');
} else {
  const batch = resolve(process.argv[2]);
  const requests = JSON.parse(await readFile(batch, 'utf8'));
  validate(requests);
  const registration = JSON.parse(await readFile(`${process.env.USERPROFILE}/.pi/agent/mcp.json`, 'utf8')).mcpServers.rea;
  const require = createRequire(registration.args[0]);
  const { Client } = require('@modelcontextprotocol/client');
  const { StdioClientTransport } = require('@modelcontextprotocol/client/stdio');
  const client = new Client({ name: 'fpilot-static-analysis', version: '1.0.0' });
  const transport = new StdioClientTransport({ command: registration.command, args: registration.args, env: { ...process.env, GHIDRA_INSTALL_DIR: 'C:/SANDBOX/APP/ghidra', JAVA_HOME: 'C:/Program Files/Eclipse Adoptium/jdk-21.0.6.7-hotspot' }, stderr: 'inherit' });
  const root = resolve('analysis/rea');
  await mkdir(root, { recursive: true });
  const snapshot = resolve('analysis/fpilot.rea-snapshot.json');
  const invoke = async (name, args) => {
    const result = await client.callTool({ name, arguments: args }, { timeout: 600000 });
    if (result.isError) {
      const error = JSON.parse(result.content.find(item => item.type === 'text').text).error;
      const id = error.details?.reported_limits?.evidence_id;
      if (error.code !== 'resource_constraint' || error.details?.resource !== 'transport' || !id) return { error };
      const exported = resolve(root, `transport-${id}.json`);
      await invoke('export_evidence_bundle', { path: exported });
      const bundle = JSON.parse(await readFile(exported, 'utf8'));
      const evidence = bundle.records.find(record => record.evidence_id === id);
      assert(evidence, 'Completed evidence missing from exported bundle');
      return evidence;
    }
    return result.structuredContent ?? result;
  };
  try {
    await client.connect(transport);
    const open = { path: resolve('FPilot.exe'), provider_id: 'ghidra' };
    try { await access(snapshot); open.snapshot_path = snapshot; } catch {}
    console.log(await invoke('open_binary', open));
    for (const q of requests) {
      const result = await invoke(q.name, q.arguments);
      const output = resolve(root, `${basename(batch, '.json')}-${q.label}.json`);
      await writeFile(output, JSON.stringify(result, null, 2), { flag: 'wx' });
      console.log(q.label, result.evidence_id ?? result.error?.code ?? '', output);
      if (q.decompile && q.name === 'resolve_containing_procedure' && result.normalized_result?.found) {
        const dossier = await invoke('analyze_function', { procedure: result.normalized_result.procedure.address });
        await writeFile(output.replace('.json', '-function.json'), JSON.stringify(dossier, null, 2), { flag: 'wx' });
        if (typeof dossier.normalized_result?.pseudocode === 'string') {
          await writeFile(output.replace('.json', '.c'), dossier.normalized_result.pseudocode, { flag: 'wx' });
        }
        console.log(q.label, 'function', dossier.evidence_id ?? dossier.error?.code);
      }
    }
    console.log(await invoke('export_evidence_bundle', { path: resolve(root, `${basename(batch, '.json')}-bundle.json`) }));
    console.log(await invoke('close_binary', { snapshot_path: snapshot, overwrite: true }));
  } finally {
    await client.close();
  }
}
