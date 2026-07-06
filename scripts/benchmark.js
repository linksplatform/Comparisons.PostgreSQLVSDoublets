// Node.js alternative benchmark script (more precise)
// Uses async/await and measures latency per request

const axios = require('axios');
const fs = require('fs');
const queries = require('../queries/simple.graphql'); // assuming graphql file loaded

const HASURA_URL = 'http://localhost:8080/v1/graphql';
const DOUBLETS_URL = 'http://localhost:8081/graphql';
const ITERATIONS = parseInt(process.argv[2]) || 100;

async function benchmark(url, query, iterations) {
  const times = [];
  for (let i = 0; i < iterations; i++) {
    const start = process.hrtime.bigint();
    try {
      await axios.post(url, { query }, {
        headers: { 'x-hasura-admin-secret': 'myadminsecret' } // only needed for Hasura
      });
    } catch (e) {
      console.error('Request failed', e.message);
    }
    const end = process.hrtime.bigint();
    times.push(Number(end - start) / 1e6); // ms
    // Small delay to avoid connection issues (Hasura issue #52 workaround)
    await new Promise(resolve => setTimeout(resolve, 10));
  }
  return times;
}

async function main() {
  const query = queries; // assuming simple query
  console.log(`Running ${ITERATIONS} iterations...`);
  
  console.log('Benchmarking Hasura...');
  const hasuraTimes = await benchmark(HASURA_URL, query, ITERATIONS);
  
  console.log('Benchmarking Doublets Gql...');
  const doubletsTimes = await benchmark(DOUBLETS_URL, query, ITERATIONS);
  
  const avg = arr => arr.reduce((a,b) => a+b, 0) / arr.length;
  console.log(`Hasura avg: ${avg(hasuraTimes).toFixed(2)} ms`);
  console.log(`Doublets avg: ${avg(doubletsTimes).toFixed(2)} ms`);
  
  const result = {
    hasura: { times: hasuraTimes, average_ms: avg(hasuraTimes) },
    doublets: { times: doubletsTimes, average_ms: avg(doubletsTimes) }
  };
  fs.writeFileSync('results/benchmark.json', JSON.stringify(result, null, 2));
}

main().catch(console.error);