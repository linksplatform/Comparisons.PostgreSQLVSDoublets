import http from 'k6/http';
import { sleep } from 'k6';
import { Trend } from 'k6/metrics';
import exec from 'k6/execution';

function boundedInteger(value, fallback, maximum) {
  const parsed = Number(value || fallback);
  if (!Number.isInteger(parsed) || parsed < 1 || parsed > maximum) {
    throw new Error(`Expected integer 1..${maximum}, got ${value}`);
  }
  return parsed;
}

const background = boundedInteger(__ENV.BACKGROUND, 3000, 3000);
const iterations = boundedInteger(__ENV.ITERATIONS, 1000, 1000);
const target = __ENV.TARGET;
if (target !== 'hasura' && target !== 'doublets') throw new Error('TARGET must be hasura or doublets');
const endpoint = `http://${target}:8080/v1/graphql`;
const headers = { 'Content-Type': 'application/json' };
const operationNames = ['Create', 'Update', 'EachAll', 'EachIdentity', 'EachConcrete', 'EachOutgoing', 'EachIncoming', 'Delete'];
const timings = Object.fromEntries(operationNames.map(name => [name, new Trend(`operation_${name}_ms`, true)]));

export const options = {
  vus: 1,
  iterations,
  maxDuration: '10m',
  setupTimeout: '5m',
  thresholds: { http_req_failed: ['rate==0'] },
  summaryTrendStats: ['count', 'avg', 'min', 'med', 'max', 'p(95)'],
};

function ensure(condition, message) {
  if (!condition) exec.test.abort(message);
}

function query(document, operation = 'setup') {
  const response = http.post(endpoint, JSON.stringify({ query: document }),
    { headers, timeout: '15s', tags: { name: operation } });
  let body;
  try { body = response.json(); } catch (_) {
    exec.test.abort(`${operation}: HTTP ${response.status}; request=${document}; response=${response.body}`);
  }
  ensure(response.status === 200 && body && body.data && !body.errors,
    `${operation}: HTTP ${response.status}; request=${document}; response=${response.body}`);
  return { data: body.data, duration: response.timings.duration };
}

function rows(result, name = 'links') {
  const value = result.data[name];
  ensure(Array.isArray(value), `Expected ${name} array: ${JSON.stringify(result.data)}`);
  return value.map(row => ({ id: Number(row.id), from_id: Number(row.from_id), to_id: Number(row.to_id) }));
}

function checkRow(row, id, from, to) {
  ensure(row && Number(row.id) === id && Number(row.from_id) === from && Number(row.to_id) === to,
    `Wrong row; expected (${id},${from},${to}), got ${JSON.stringify(row)}`);
}

function mutate(document, name, operation) {
  const result = query(document, operation);
  const mutation = result.data[name];
  ensure(mutation && mutation.affected_rows === 1 && mutation.returning.length === 1,
    `${operation}: expected one affected/returned row: ${JSON.stringify(result.data)}`);
  return { row: mutation.returning[0], duration: result.duration };
}

export function setup() {
  // Wait on these disposable internal services; no external endpoint can be selected.
  let ready = false;
  for (let attempt = 0; attempt < 90; attempt++) {
    const response = http.post(endpoint, JSON.stringify({ query: '{__typename}' }),
      { headers, timeout: '2s', tags: { name: 'readiness' }, responseCallback: null });
    if (response.status === 200) { ready = true; break; }
    sleep(1);
  }
  ensure(ready, `${target} did not become ready`);
  if (target === 'hasura') {
    const response = http.post('http://hasura:8080/v1/metadata', JSON.stringify({
      type: 'pg_track_table', args: { source: 'default', table: { schema: 'public', name: 'links' } },
    }), { headers });
    ensure(response.status === 200, `Cannot track local links table: ${response.body}`);
  }
  ensure(rows(query('{links(limit:1){id from_id to_id}}')).length === 0,
    'Benchmark requires an empty dedicated database; refusing to modify an existing dataset');
  for (let start = 1; start <= background; start += 100) {
    const objects = [];
    for (let id = start; id <= Math.min(start + 99, background); id++) objects.push(`{from_id:${id},to_id:${id}}`);
    const result = query(`mutation{insert_links(objects:[${objects.join(',')}]){affected_rows returning{id from_id to_id}}}`);
    const inserted = result.data.insert_links;
    ensure(inserted.affected_rows === objects.length && inserted.returning.length === objects.length, 'Background row count mismatch');
    inserted.returning.forEach((row, index) => checkRow(row, start + index, start + index, start + index));
  }
  const seeded = rows(query('{links{id from_id to_id}}'));
  ensure(seeded.length === background, 'Seeded database cardinality mismatch');
  seeded.sort((a, b) => a.id - b.id).forEach((row, index) => checkRow(row, index + 1, index + 1, index + 1));
  return { seeded: background };
}

export default function () {
  // Create a point through the same two HTTP mutations on both servers.
  const created = mutate('mutation{insert_links(objects:[{from_id:0,to_id:0}]){affected_rows returning{id from_id to_id}}}',
    'insert_links', 'Create');
  const id = Number(created.row.id);
  ensure(Number.isSafeInteger(id) && id > background, 'New link ID must be outside the background dataset');
  const pointed = mutate(`mutation{update_links(where:{id:{_eq:${id}}},_set:{from_id:${id},to_id:${id}}){affected_rows returning{id from_id to_id}}}`,
    'update_links', 'Create');
  checkRow(pointed.row, id, id, id);
  timings.Create.add(created.duration + pointed.duration);

  const updated = mutate(`mutation{update_links(where:{id:{_eq:${id}}},_set:{from_id:${id},to_id:1}){affected_rows returning{id from_id to_id}}}`,
    'update_links', 'Update');
  checkRow(updated.row, id, id, 1);
  timings.Update.add(updated.duration);

  const reads = [
    ['EachAll', '{links{id from_id to_id}}', background + 1],
    ['EachIdentity', `{links(where:{id:{_eq:${id}}}){id from_id to_id}}`, 1],
    ['EachConcrete', `{links(where:{from_id:{_eq:${id}},to_id:{_eq:1}}){id from_id to_id}}`, 1],
    ['EachOutgoing', `{links(where:{from_id:{_eq:${id}}}){id from_id to_id}}`, 1],
    ['EachIncoming', '{links(where:{to_id:{_eq:1}}){id from_id to_id}}', 2],
  ];
  for (const [name, document, count] of reads) {
    const result = query(document, name);
    const actual = rows(result);
    ensure(new Set(actual.map(row => row.id)).size === actual.length, `${name}: duplicate row IDs`);
    ensure(actual.length === count, `${name}: expected ${count} rows, got ${actual.length}`);
    actual.forEach(row => {
      if (row.id === id) checkRow(row, id, id, 1);
      else { ensure((name === 'EachAll' && row.id >= 1 && row.id <= background) || (name === 'EachIncoming' && row.id === 1), `${name}: unexpected row`); checkRow(row, row.id, row.id, row.id); }
    });
    ensure(actual.some(row => row.id === id), `${name}: current link is missing`);
    timings[name].add(result.duration);
  }

  const removed = mutate(`mutation{delete_links(where:{id:{_eq:${id}}}){affected_rows returning{id from_id to_id}}}`,
    'delete_links', 'Delete');
  checkRow(removed.row, id, id, 1);
  ensure(rows(query(`{links(where:{id:{_eq:${id}}}){id from_id to_id}}`, 'verify-delete')).length === 0,
    'Deleted link still exists');
  timings.Delete.add(removed.duration);
}

export function handleSummary(data) {
  const measured = Object.fromEntries(operationNames.map(name => [name, data.metrics[`operation_${name}_ms`]?.values]));
  const complete = operationNames.every(name => measured[name] && measured[name].count === iterations);
  const report = { target, background, iterations, complete, unit: 'milliseconds',
    create_http_requests: 2, other_operation_http_requests: 1, operations: measured };
  return { [`/results/${target}.json`]: JSON.stringify(report, null, 2),
    stdout: JSON.stringify(report, null, 2) + '\n' };
}
