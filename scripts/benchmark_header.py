#!/usr/bin/env python3
"""Write provenance before benchmark output without printing connection credentials."""
import argparse
import datetime
import os
import platform
import re
import tomllib
import xml.etree.ElementTree as ET
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('language', choices=['rust', 'csharp'])
parser.add_argument('backend', choices=['psql', 'doublets'])
options = parser.parse_args()
root = Path(__file__).resolve().parents[1]
if options.language == 'rust':
    packages = tomllib.loads((root / 'rust/Cargo.lock').read_text())['package']
    versions = {package['name']: package['version'] for package in packages}
    driver, library = 'postgres ' + versions['postgres'], 'doublets ' + versions['doublets']
else:
    project = ET.parse(root / 'csharp/PostgreSQLVSDoublets/PostgreSQLVSDoublets.csproj')
    versions = {p.attrib['Include']: p.attrib['Version'] for p in project.findall('.//PackageReference')}
    driver, library = 'Npgsql ' + versions['Npgsql'], 'Platform.Data.Doublets ' + versions['Platform.Data.Doublets']
cpu = platform.processor() or platform.machine()
if Path('/proc/cpuinfo').exists():
    match = re.search(r'model name\s*:\s*(.*)', Path('/proc/cpuinfo').read_text())
    if match:
        cpu = match.group(1)
metadata = {
    'postgresql': os.getenv('POSTGRES_VERSION', 'unknown version'),
    'driver': driver,
    'doublets': library,
    'cpu': cpu,
    'date': datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds'),
    'links': os.getenv('BENCHMARK_LINKS', '100'),
    'background': os.getenv('BENCHMARK_BACKGROUND_LINKS', '1000'),
}
if os.getenv('GITHUB_RUN_ID'):
    metadata['run'] = f"{os.getenv('GITHUB_SERVER_URL', 'https://github.com')}/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{os.environ['GITHUB_RUN_ID']}"
for key, value in metadata.items():
    print(f'# {key}: {value}')
