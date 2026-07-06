const { runBenchmark } = require('graphql-bench');
const fs = require('fs');
const path = require('path');

async function main() {
  const configPath = path.join(__dirname, 'config', 'config.yml');
  const config = fs.readFileSync(configPath, 'utf8');
  const options = {
    config: config,
    outputDir: path.join(__dirname, 'results'),
  };
  await runBenchmark(options);
}

main().catch(console.error);
