#!/usr/bin/env python3
"""Record non-secret provenance for a local benchmark run."""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys


def main():
    directory = Path(__file__).resolve().parent
    output = Path(sys.argv[1])
    references = [line.split('image:', 1)[1].strip() for line in (directory / 'compose.yaml').read_text().splitlines()
                  if line.lstrip().startswith('image:')]
    images = subprocess.check_output(['docker', 'image', 'inspect', *references], text=True)
    image_info = [{'tags': image['RepoTags'], 'digests': image['RepoDigests'], 'id': image['Id']}
                  for image in json.loads(images)]
    record = {
        'started_at_utc': datetime.now(timezone.utc).isoformat(),
        'doublets_commit': 'b11f33b4080a7ef6b6d1c056c40bbf758d6cdd7e',
        'settings_commit': 'f08ce8fc84f8ab7ccdcf9293b5566007305254c1',
        'benchmark_sha256': hashlib.sha256((directory / 'benchmark.js').read_bytes()).hexdigest(),
        'compose_sha256': hashlib.sha256((directory / 'compose.yaml').read_bytes()).hexdigest(),
        'doublets_dockerfile': (directory / 'Dockerfile.doublets').read_text(),
        'platform': platform.platform(), 'logical_cpus': os.cpu_count(),
        'docker_version': subprocess.check_output(['docker', 'version', '--format', '{{.Server.Version}}'], text=True).strip(),
        'background': int(os.environ.get('BACKGROUND', 3000)),
        'iterations': int(os.environ.get('ITERATIONS', 1000)),
        'server_order': ['hasura', 'doublets'], 'virtual_users': 1,
        'images': image_info,
        'measurement': 'HTTP response duration, milliseconds; Create sums two requests',
    }
    (output / 'run.json').write_text(json.dumps(record, indent=2) + '\n')


if __name__ == '__main__':
    main()
