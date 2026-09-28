#!/usr/bin/env python3
"""Compare only complete, compatible, measured benchmark summaries."""
import argparse
import json
import math
from pathlib import Path

OPERATIONS = ('Create', 'Update', 'EachAll', 'EachIdentity', 'EachConcrete', 'EachOutgoing', 'EachIncoming', 'Delete')


def validate(result, target):
    if result.get('target') != target or result.get('complete') is not True or result.get('unit') != 'milliseconds':
        raise ValueError(f'{target}: incorrect target/unit or incomplete run')
    background = result.get('background')
    if type(background) is not int or not 1 <= background <= 3000:
        raise ValueError(f'{target}: invalid background count')
    if result.get('create_http_requests') != 2 or result.get('other_operation_http_requests') != 1:
        raise ValueError(f'{target}: unexpected HTTP operation counts')
    iterations = result.get('iterations')
    if type(iterations) is not int or not 1 <= iterations <= 1000:
        raise ValueError(f'{target}: invalid iteration count')
    for name in OPERATIONS:
        metric = result.get('operations', {}).get(name, {})
        if type(metric.get('count')) is not int or metric.get('count') != iterations:
            raise ValueError(f'{target}: incomplete {name} samples')
        for statistic in ['avg', 'med', 'p(95)']:
            value = metric.get(statistic)
            if type(value) not in [int, float] or not math.isfinite(value) or value < 0:
                raise ValueError(f'{target}: invalid {name} {statistic}')


def compare(hasura, doublets):
    validate(hasura, 'hasura')
    validate(doublets, 'doublets')
    for key in ['background', 'iterations', 'create_http_requests', 'other_operation_http_requests']:
        if hasura.get(key) != doublets.get(key):
            raise ValueError(f'Incompatible runs: {key} differs')
    lines = [
        '# Measured GraphQL comparison', '',
        f"{hasura['background']} background point links; {hasura['iterations']} sequential iterations per server; one virtual user.",
        'Times are HTTP response duration in milliseconds, not database-only time.',
        'Create includes two HTTP mutations on both servers; other operations use one.', '',
        '| Operation | Hasura mean | Doublets mean | Hasura p95 | Doublets p95 |',
        '|---|---:|---:|---:|---:|',
    ]
    for operation in OPERATIONS:
        left, right = hasura['operations'][operation], doublets['operations'][operation]
        lines.append(f"| {operation} | {left['avg']:.4f} | {right['avg']:.4f} | {left['p(95)']:.4f} | {right['p(95)']:.4f} |")
    lines.extend(['', 'Each response was checked for GraphQL errors, row count and returned values.',
                  'These local measurements describe this run and host; repeat runs before drawing performance conclusions.'])
    return '\n'.join(lines) + '\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('hasura', type=Path)
    parser.add_argument('doublets', type=Path)
    args = parser.parse_args()
    try:
        print(compare(json.loads(args.hasura.read_text()), json.loads(args.doublets.read_text())), end='')
    except (ValueError, KeyError, TypeError) as error:
        parser.exit(1, f'Cannot compare results: {error}\n')


if __name__ == '__main__':
    main()
