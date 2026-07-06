import json
import time
import requests

class Benchmark:
    def __init__(self, config_path='config.json'):
        with open(config_path) as f:
            self.config = json.load(f)
        self.hasura_endpoint = self.config['hasura_endpoint']
        self.doublets_endpoint = self.config['doublets_endpoint']
        self.iterations = self.config['iterations']
        self.warmup_iterations = self.config['warmup_iterations']

    def run_query(self, endpoint, query, variables=None):
        payload = {'query': query}
        if variables:
            payload['variables'] = variables
        response = requests.post(endpoint, json=payload)
        return response.elapsed.total_seconds()

    def benchmark(self):
        # Define queries
        hasura_query = """
        query {
          links {
            from
            to
          }
        }
        """
        doublets_query = """
        query {
          links {
            from
            to
          }
        }
        """
        # Warmup
        for _ in range(self.warmup_iterations):
            self.run_query(self.hasura_endpoint, hasura_query)
            self.run_query(self.doublets_endpoint, doublets_query)

        # Benchmark
        hasura_times = []
        doublets_times = []
        for _ in range(self.iterations):
            t = self.run_query(self.hasura_endpoint, hasura_query)
            hasura_times.append(t)
            t = self.run_query(self.doublets_endpoint, doublets_query)
            doublets_times.append(t)

        # Results
        results = {
            'hasura': {
                'avg': sum(hasura_times)/len(hasura_times),
                'min': min(hasura_times),
                'max': max(hasura_times),
                'times': hasura_times
            },
            'doublets': {
                'avg': sum(doublets_times)/len(doublets_times),
                'min': min(doublets_times),
                'max': max(doublets_times),
                'times': doublets_times
            }
        }
        return results

if __name__ == '__main__':
    bench = Benchmark()
    results = bench.benchmark()
    print(json.dumps(results, indent=2))
