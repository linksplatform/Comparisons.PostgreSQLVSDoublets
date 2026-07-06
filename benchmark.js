const fetch = require('node-fetch');
const fs = require('fs');
const path = require('path');

const config = JSON.parse(fs.readFileSync('config.json', 'utf8'));

async function benchmarkEndpoint(endpointConfig, query) {
  const url = endpointConfig.url;
  const headers = {
    'Content-Type': 'application/json',
    ...endpointConfig.headers
  };

  const times = [];

  for (let i = 0; i < config.runs; i++) {
    const start = process.hrtime.bigint();
    try {
      const response = await fetch(url, {
        method: 'POST',
        headers,
        body: JSON.stringify({ query })
      });
      await response.json(); // consume response
    } catch (e) {
      console.error('Request failed:', e);
      continue;
    }
    const end = process.hrtime.bigint();
    times.push(Number(end - start) / 1e6); // in milliseconds
  }

  return times;
}

function analyze(times) {
  const n = times.length;
  if (n === 0) return { mean: NaN, min: NaN, max: NaN, stddev: NaN };
  const sum = times.reduce((a, b) => a + b, 0);
  const mean = sum / n;
  const min = Math.min(...times);
  const max = Math.max(...times);
  const variance = times.reduce((acc, t) => acc + (t - mean) ** 2, 0) / n;
  const stddev = Math.sqrt(variance);
  return { mean, min, max, stddev, runs: n };
}

async function main() {
  const queryDir = 'queries';
  const queryFiles = config.queries.map(q => path.join(queryDir, q));

  for (const file of queryFiles) {
    if (!fs.existsSync(file)) {
      console.warn(`Query file ${file} not found. Skipping.`);
      continue;
    }
    const query = fs.readFileSync(file, 'utf8').trim();
    console.log(`\nBenchmarking query from ${file}:`);
    console.log(query);

    console.log('\n--- Hasura ---');
    const hasuraTimes = await benchmarkEndpoint(config.hasura, query);
    const hasuraStats = analyze(hasuraTimes);
    console.log(`Mean: ${hasuraStats.mean.toFixed(2)} ms`);
    console.log(`Min: ${hasuraStats.min.toFixed(2)} ms`);
    console.log(`Max: ${hasuraStats.max.toFixed(2)} ms`);
    console.log(`StdDev: ${hasuraStats.stddev.toFixed(2)} ms`);
    console.log(`Runs: ${hasuraStats.runs}`);

    console.log('\n--- Doublets ---');
    const doubletsTimes = await benchmarkEndpoint(config.doublets, query);
    const doubletsStats = analyze(doubletsTimes);
    console.log(`Mean: ${doubletsStats.mean.toFixed(2)} ms`);
    console.log(`Min: ${doubletsStats.min.toFixed(2)} ms`);
    console.log(`Max: ${doubletsStats.max.toFixed(2)} ms`);
    console.log(`StdDev: ${doubletsStats.stddev.toFixed(2)} ms`);
    console.log(`Runs: ${doubletsStats.runs}`);

    // Optional: compute ratio
    if (hasuraStats.mean && doubletsStats.mean) {
      const ratio = doubletsStats.mean / hasuraStats.mean;
      console.log(`\nRatio (Doublets/Hasura): ${ratio.toFixed(2)}`);
    }
  }
}

main().catch(console.error);