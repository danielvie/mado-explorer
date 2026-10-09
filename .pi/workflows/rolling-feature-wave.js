export const meta = {
  name: 'rolling-feature-wave',
  description: 'Implement three isolated UI behaviors and independently review each frozen candidate; integration stays supervisor-owned',
  phases: [
    { title: 'Implement', detail: 'Three owned worktrees implement controls and focused regression checks' },
    { title: 'Review', detail: 'Each terminal candidate receives a fresh read-only review immediately' }
  ]
}
if (!Array.isArray(args?.slices) || args.slices.length < 1 || args.slices.length > 3) {
  throw new Error('A feature wave requires one to three supervisor-commissioned independent slices.');
}
if (new Set(args.slices.map(s => s.root)).size !== args.slices.length) {
  throw new Error('Feature workers require distinct owned checkouts.');
}
const CANDIDATE = { type: 'object', properties: {
  slice: {type:'string'}, status:{type:'string',enum:['ready','blocked','partial']},
  head:{type:'string'}, changed_paths:{type:'array',items:{type:'string'}},
  checks:{type:'array',items:{type:'string'}}, partial_ids:{type:'array',items:{type:'string'}},
  blockers:{type:'array',items:{type:'string'}}, handoff:{type:'string'}
}, required:['slice','status','head','changed_paths','checks','partial_ids','blockers','handoff'], additionalProperties:false };
const REVIEW = { type:'object', properties:{ head:{type:'string'}, verdict:{type:'string',enum:['no-concrete-blocker','needs-fix','insufficient-evidence']},
  findings:{type:'array',items:{type:'object',properties:{title:{type:'string'},evidence:{type:'string'},scenario:{type:'string'},blocking:{type:'boolean'}},required:['title','evidence','scenario','blocking'],additionalProperties:false}},
  limits:{type:'array',items:{type:'string'}}
},required:['head','verdict','findings','limits'],additionalProperties:false };
const common = `Read AGENTS.md, SPEC.md, CONTEXT.md and the supervisor commission at the top of your worktree PROGRESS.md. Exact cwd must be explicit in shell commands. No nested delegation. No emojis. Preserve all preexisting work. One writer per checkout. Root main is unavailable; no main/root/sibling edits. No personal filesystem, interactive clipboard, elevation or foreground probes. These are authorized isolated preparation only; production adapters and later checkpoint enablement remain supervisor-owned and gated. Implement actual functioning egui controls, not drawn placeholders or a model with no UI. Use installed matching egui from root Cargo.lock and standard library; no extra dependencies without concrete need. Each component is a standalone Rust library with tracked Cargo.lock and Taskfile default --list/run safe generated demonstration/test/clean confined to its root target output, each task with desc. Minimal bounded component state and one useful focused headless regression suite. Do not build generic frameworks. Read source completely before relying on it. Run formatting, locked incremental all-target compile, focused tests and diff check after a coherent change; do not run whole root suite/Clippy. After two unsuccessful focused fixes for one failure, report preserved partial work and exact failure, do not weaken assertions or silently switch tools/models. Worker may commit only its own component. Parent-owned PROGRESS.md commission note is intentionally modified, must remain unstaged/unedited; report it separately from component cleanliness. Return candidate SHA and durable component HANDOFF.md with exact commands/outcomes, public API, UI/state coverage, partial acceptance IDs, limits and smallest serial integration follow-up. No complete criterion or runtime verification claim from a partial control.`;
log('Dispatching three feature workers. Reviews start per candidate without a round barrier. No automatic integration, enablement or acceptance.');
const results = await pipeline(args.slices,
  s => agent(`${common}\n\nSlice ${s.id}. Exact checkout ${s.root}, branch lane/${s.name}, accepted baseline3679878. Exclusive writes ${s.boundary}.\n${s.implementation}`, {agentType:'worker',label:`implement:${s.id}`,phase:'Implement',schema:CANDIDATE}),
  async (candidate,s) => {
    if (!candidate) { log(`${s.id}: implementation returned no handoff; preserve checkout and reconcile before retry.`); return {slice:s.id,status:'incomplete'}; }
    if (candidate.status !== 'ready' || !/^[0-9a-f]{40}$/.test(candidate.head)) { log(`${s.id}: ${candidate.status}; no promotion or automatic rework.`); return {slice:s.id,candidate,review:null}; }
    let review = await agent(`Fresh read-only review, no delegation, edits, commits, builds, tests, app launches or data/clipboard probes. Exact checkout ${s.root}. Read local AGENTS/SPEC/CONTEXT/PROGRESS and complete ${s.boundary.replace('/**','')}/HANDOFF.md, source/tests/manifest/Taskfile. Use git to confirm exact HEAD ${candidate.head}, component-only committed diff and intentional supervisor-only unstaged PROGRESS.md note. Scope is ${s.id}: ${s.implementation}. Review actual UI/state behavior and focused checks, especially stale/closed request routing, pane/document independence, malformed geometry/metadata, keyboard focus/Cancel/defaults and bounded allocations. Identify concrete correctness/security/data-loss/contract findings with source line evidence and reproducible scenario, not style or speculative features. Separate recorded tests from verification performed here. This review must not approve production adapters, feature enablement or complete acceptance. Return exact-head verdict with at most 450 words in findings/limits. Candidate: ${JSON.stringify(candidate)}`, {agentType:'skeptic',label:`review:${s.id}`,phase:'Review',schema:REVIEW});
    if (review && review.head !== candidate.head) {
      log(`${s.id}: review head mismatch; candidate unapproved.`);
      review = null;
    }
    if (!review) log(`${s.id}: review missing; candidate unapproved.`);
    else if (review.verdict !== 'no-concrete-blocker') log(`${s.id}: ${review.verdict}; fresh fix worker required after supervisor classification.`);
    else log(`${s.id}: read-only review found no concrete blocker; awaits supervisor inspection and serial integration.`);
    return {slice:s.id,candidate,review};
  }
);
return {results:results.filter(Boolean), integration_performed:false, acceptance_updated:false, production_enablement:false};