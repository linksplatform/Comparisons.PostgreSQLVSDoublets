import tempfile
import unittest
from pathlib import Path

import benchmark_report as report


class ReportTests(unittest.TestCase):
    def test_comparisons(self):
        cases = [((50, 0), (100, 0), '2× faster'),
                 ((200, 0), (100, 0), '2× slower'),
                 ((104, 0), (100, 0), '≈ same'),
                 ((100, 0), (105, 0), '≈ same'),
                 ((70, 20), (100, 15), '≈ same')]
        for measured, baseline, expected in cases:
            with self.subTest(measured=measured):
                self.assertEqual(report.comparison(measured, baseline), expected)

    def test_parse_deviation_and_commas(self):
        times, metadata = report.parse('# cpu: Example CPU\ntest Create/PSQL_Transaction ... bench: 1,234 ns/iter (+/- 56)')
        self.assertEqual(times[('Create', 'PSQL_Transaction')], (1234, 56))
        self.assertEqual(metadata['cpu'], 'Example CPU')

    def test_reject_invalid_results(self):
        for line in ['test Create/PSQL_Transaction ... ERROR',
                     'test Unknown/PSQL_Transaction ... bench: 10 ns/iter (+/- 0)',
                     'test Create/Unknown ... bench: 10 ns/iter (+/- 0)',
                     'test Create/PSQL_Transaction ... bench: 0 ns/iter (+/- 0)']:
            with self.subTest(line=line), self.assertRaises(ValueError):
                report.parse(line)

    def test_duplicate_is_rejected(self):
        line = 'test Create/PSQL_Transaction ... bench: 10 ns/iter (+/- 0)'
        with self.assertRaises(ValueError):
            report.parse(line + '\n' + line)

    def test_markers_preserve_surrounding_text(self):
        doc = 'before\n<!-- results:start -->\nstale\n<!-- results:end -->\nafter'
        self.assertEqual(report.replace_section(doc, 'new'), doc.replace('stale', 'new'))
        for invalid in ['no markers', '<!-- results:end --><!-- results:start -->', doc + doc]:
            with self.assertRaises(ValueError):
                report.replace_section(invalid, 'new')

    def test_missing_backend_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'rust-100-doublets.txt'
            path.write_text(self.output('doublets'))
            with self.assertRaises(ValueError):
                report.load(directory)

    @staticmethod
    def output(backend, links=10):
        lines = [f'# links: {links}', '# background: 100', '# cpu: Test CPU',
                 '# date: 2026-10-06', '# run: https://github.com/example/actions/runs/1',
                 '# postgresql: 17.6', '# driver: test-driver 1', '# doublets: test-doublets 2']
        lines += [f'test {op.replace(" ", "_")}/{implementation} ... bench: 100 ns/iter (+/- 10)'
                  for op in report.OPERATIONS for implementation, _, _ in report.BACKENDS[backend]]
        return '\n'.join(lines)

    def test_complete_report_and_mismatched_sizes(self):
        with tempfile.TemporaryDirectory() as directory:
            for backend in report.BACKENDS:
                (Path(directory) / f'rust-100-{backend}.txt').write_text(self.output(backend))
            results = report.load(directory)
            generated = report.section(results, None)
            for text in ['### Rust', '### C#', '≈ same', 'Test CPU', '17.6', 'GitHub Actions run']:
                self.assertIn(text, generated)
            (Path(directory) / 'rust-100-psql.txt').write_text(self.output('psql', 20))
            with self.assertRaises(ValueError):
                report.section(report.load(directory), None)

    def test_incomplete_table_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            for backend in report.BACKENDS:
                (Path(directory) / f'rust-100-{backend}.txt').write_text(self.output(backend).rsplit('\n', 1)[0])
            with self.assertRaises(ValueError):
                report.load(directory)

    def test_reject_mismatched_provenance(self):
        for old, new in [('# background: 100', '# background: 999'),
                         ('# cpu: Test CPU', 'no cpu metadata'),
                         ('# links: 10', '# links: 0')]:
            with self.subTest(old=old), tempfile.TemporaryDirectory() as directory:
                for backend in report.BACKENDS:
                    output = self.output(backend)
                    if backend == 'psql':
                        output = output.replace(old, new)
                    (Path(directory) / f'rust-100-{backend}.txt').write_text(output)
                with self.assertRaises(ValueError):
                    report.load(directory)

    def test_fastest_postgresql_baseline(self):
        times = {(op, implementation): (200, 0)
                 for op in report.OPERATIONS for implementation, _, _ in report.IMPLEMENTATIONS}
        times.update({(op, 'PSQL_Transaction'): (100, 0) for op in report.OPERATIONS})
        self.assertIn('2× slower', report.table(times))
        self.assertIn('**100 ns**', report.table(times))


if __name__ == '__main__':
    unittest.main()
