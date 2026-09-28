import unittest
from report import compare, OPERATIONS


def fixture(target):
    return {'target': target, 'complete': True, 'unit': 'milliseconds', 'background': 3000,
            'iterations': 2, 'create_http_requests': 2, 'other_operation_http_requests': 1,
            'operations': {name: {'count': 2, 'avg': 1.0, 'med': 1.0, 'p(95)': 2.0} for name in OPERATIONS}}


class ReportTests(unittest.TestCase):
    def test_complete_synthetic_fixture_formats(self):
        self.assertIn('| EachIdentity |', compare(fixture('hasura'), fixture('doublets')))

    def test_rejects_incomplete(self):
        left = fixture('hasura')
        left['complete'] = False
        with self.assertRaises(ValueError): compare(left, fixture('doublets'))

    def test_rejects_failed_sample_count(self):
        left = fixture('hasura')
        left['operations']['Delete']['count'] = 1
        with self.assertRaises(ValueError): compare(left, fixture('doublets'))

    def test_rejects_mismatched_background(self):
        right = fixture('doublets')
        right['background'] = 100
        with self.assertRaises(ValueError): compare(fixture('hasura'), right)

    def test_rejects_invalid_number(self):
        for value in [float('nan'), float('inf'), -1, None, True]:
            left = fixture('hasura')
            left['operations']['Create']['avg'] = value
            with self.assertRaises(ValueError): compare(left, fixture('doublets'))

    def test_rejects_invalid_background_or_request_count(self):
        for key, value in [('background', -1), ('background', True), ('create_http_requests', 1)]:
            left = fixture('hasura'); left[key] = value
            with self.assertRaises(ValueError): compare(left, fixture('doublets'))

    def test_rejects_wrong_target_or_unit(self):
        for key, value in [('target', 'doublets'), ('unit', 'nanoseconds')]:
            left = fixture('hasura'); left[key] = value
            with self.assertRaises(ValueError): compare(left, fixture('doublets'))


if __name__ == '__main__':
    unittest.main()
