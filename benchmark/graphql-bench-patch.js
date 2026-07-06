// Workaround for hasura/graphql-bench issue #52: 
// The tool does not support header variables properly.
// This patch modifies the internal HTTP request to include custom headers.
// Apply before running: node patch.js

const fs = require('fs');
const path = require('path');

const benchDir = path.join(__dirname, 'node_modules', 'graphql-bench');
const runnerFile = path.join(benchDir, 'dist', 'runner.js');

if (!fs.existsSync(runnerFile)) {
  console.error('Runner file not found. Is graphql-bench installed?');
  process.exit(1);
}

let content = fs.readFileSync(runnerFile, 'utf8');

// Patch to include custom headers from config
content = content.replace(
  /const headers = /g,
  `const headers = { ...(config.headers || {}), `
);

fs.writeFileSync(runnerFile, content);
console.log('Patched runner.js for custom headers support.');
