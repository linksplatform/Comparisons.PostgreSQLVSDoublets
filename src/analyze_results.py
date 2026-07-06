import csv
import sys

def analyze(csv_file):
    results = {}
    with open(csv_file, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            system = row['System']
            op = row['Operation']
            duration = float(row['Duration (s)'])
            if system not in results:
                results[system] = {}
            results[system][op] = duration

    print("Comparison Results (average of single operations):")
    print(f"{'Operation':<10} {'Hasura (s)':<15} {'Doublets (s)':<15} {'Ratio (H/D)':<10}")
    for op in ['insert', 'query', 'update', 'delete']:
        h = results.get('Hasura', {}).get(op, 0)
        d = results.get('Doublets', {}).get(op, 0)
        ratio = h / d if d != 0 else float('inf')
        print(f"{op:<10} {h:<15.6f} {d:<15.6f} {ratio:<10.2f}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        analyze(sys.argv[1])
    else:
        analyze("results.csv")
