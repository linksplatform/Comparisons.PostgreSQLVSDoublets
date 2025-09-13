#!/usr/bin/env python3
"""
Process k6 GraphQL benchmark results and generate visualizations
Similar to the existing rust/out.py script but for GraphQL benchmarks
"""

import json
import matplotlib.pyplot as plt
import numpy as np
import sys
from pathlib import Path

def load_k6_results(file_path):
    """Load k6 JSON results and extract metrics"""
    with open(file_path, 'r') as f:
        data = json.load(f)
    
    metrics = {}
    
    # Extract HTTP request duration metrics by group
    for metric_name, metric_data in data['metrics'].items():
        if 'http_req_duration' in metric_name:
            group_name = metric_name.split('{')[1].split(':')[1].split('}')[0] if '{' in metric_name else 'total'
            if 'values' in metric_data:
                # Get p95 (95th percentile) duration
                p95_duration = metric_data['values']['p(95)']
                metrics[group_name] = p95_duration * 1000000  # Convert to nanoseconds
    
    return metrics

def create_comparison_chart(hasura_results, doublets_results, output_file):
    """Create comparison chart similar to existing benchmark visualizations"""
    
    # Operations mapping
    operation_mapping = {
        'Create Operations': 'Create',
        'Update Operations': 'Update', 
        'Delete Operations': 'Delete',
        'Read Operations': 'Read (All types)'
    }
    
    operations = []
    hasura_times = []
    doublets_times = []
    speedup_factors = []
    
    for group_name in hasura_results:
        if group_name in doublets_results:
            op_name = operation_mapping.get(group_name, group_name)
            operations.append(op_name)
            
            hasura_time = hasura_results[group_name]
            doublets_time = doublets_results[group_name]
            
            hasura_times.append(hasura_time)
            doublets_times.append(doublets_time)
            
            speedup = hasura_time / doublets_time if doublets_time > 0 else 0
            speedup_factors.append(speedup)
    
    # Create figure with subplots
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 8))
    
    # Linear scale chart
    x = np.arange(len(operations))
    width = 0.35
    
    bars1 = ax1.bar(x - width/2, hasura_times, width, label='PostgreSQL + Hasura', alpha=0.8, color='#1f77b4')
    bars2 = ax1.bar(x + width/2, doublets_times, width, label='Doublets + GQL', alpha=0.8, color='#ff7f0e')
    
    ax1.set_xlabel('Operations')
    ax1.set_ylabel('Response Time (nanoseconds)')
    ax1.set_title('GraphQL Performance Comparison (Linear Scale)')
    ax1.set_xticks(x)
    ax1.set_xticklabels(operations, rotation=45, ha='right')
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    # Logarithmic scale chart  
    bars3 = ax2.bar(x - width/2, hasura_times, width, label='PostgreSQL + Hasura', alpha=0.8, color='#1f77b4')
    bars4 = ax2.bar(x + width/2, doublets_times, width, label='Doublets + GQL', alpha=0.8, color='#ff7f0e')
    
    ax2.set_xlabel('Operations')
    ax2.set_ylabel('Response Time (nanoseconds, log scale)')
    ax2.set_title('GraphQL Performance Comparison (Log Scale)')
    ax2.set_xticks(x)
    ax2.set_xticklabels(operations, rotation=45, ha='right')
    ax2.set_yscale('log')
    ax2.legend()
    ax2.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    
    return speedup_factors

def generate_results_table(hasura_results, doublets_results, speedup_factors):
    """Generate results table similar to the README format"""
    
    operation_mapping = {
        'Create Operations': 'Create',
        'Update Operations': 'Update',
        'Delete Operations': 'Delete', 
        'Read Operations': 'Read (All types)'
    }
    
    print("\n### GraphQL Benchmark Results (all numbers are in nanoseconds)\n")
    print("| Operation | PostgreSQL + Hasura | Doublets + GQL | Speedup Factor |")
    print("|-----------|---------------------|----------------|----------------|")
    
    for i, group_name in enumerate(hasura_results):
        if group_name in doublets_results:
            op_name = operation_mapping.get(group_name, group_name)
            hasura_time = int(hasura_results[group_name])
            doublets_time = int(doublets_results[group_name])
            speedup = speedup_factors[i] if i < len(speedup_factors) else 0
            
            print(f"| {op_name} | {hasura_time:,} | {doublets_time:,} | {speedup:.1f}x faster |")

def main():
    if len(sys.argv) != 3:
        print("Usage: python process-results.py <hasura_results.json> <doublets_results.json>")
        sys.exit(1)
    
    hasura_file = sys.argv[1]
    doublets_file = sys.argv[2]
    
    # Load results
    hasura_results = load_k6_results(hasura_file)
    doublets_results = load_k6_results(doublets_file)
    
    # Generate charts
    speedup_factors = create_comparison_chart(
        hasura_results, 
        doublets_results, 
        'bench_graphql.png'
    )
    
    create_comparison_chart(
        hasura_results,
        doublets_results, 
        'bench_graphql_log_scale.png'
    )
    
    # Generate results table
    generate_results_table(hasura_results, doublets_results, speedup_factors)
    
    print(f"\nCharts saved as 'bench_graphql.png' and 'bench_graphql_log_scale.png'")

if __name__ == "__main__":
    main()