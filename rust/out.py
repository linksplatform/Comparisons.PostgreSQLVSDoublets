import re
import matplotlib.pyplot as plt
import numpy as np

data = open("out.txt").read()

patterns = [
    r"test\s+(\w+)/(PSQL)_(\w+)\s+\.\.\.\s+bench:\s+(\d+)\s+ns/iter\s+\(\+/-\s+\d+\)",
    r"test\s+(\w+)/(Doublets)_(\w+)_(\w+)\s+\.\.\.\s+bench:\s+(\d+)\s+ns/iter\s+\(\+/-\s+\d+\)"
]
PSQL_Transaction = []
PSQL_NonTransaction = []
Doublets_United_Volatile = []
Doublets_United_NonVolatile = []
Doublets_Split_Volatile = []
Doublets_Split_NonVolatile = []

for pattern in patterns:
    matches = re.findall(pattern, data)
    for match in matches:
        if match[1] == 'PSQL':
            method, _category, transaction, time = match
            if transaction == "Transaction":
                PSQL_Transaction.append(int(time))
            else:
                PSQL_NonTransaction.append(int(time))
        else:
            method, _category, trees, storage, time = match
            if trees == 'United':
                if storage == 'Volatile':
                    Doublets_United_Volatile.append(int(time))
                else:
                    Doublets_United_NonVolatile.append(int(time))
            else:
                if storage == 'Volatile':
                    Doublets_Split_Volatile.append(int(time))
                else:
                    Doublets_Split_NonVolatile.append(int(time))

labels = ['Create', 'Delete', 'Each Identity', 'Each Concrete', 'Each Outgoing', 'Each Incoming', 'Each All', 'Update']


def bench1():
    # Scale raw ns values for visualization purposes (divided by 10,000,000)
    Doublets_United_Volatile_Timings = [max(1, x // 10000000) for x in Doublets_United_Volatile]
    Doublets_United_NonVolatile_Timings = [max(1, x // 10000000) for x in Doublets_United_NonVolatile]
    Doublets_Split_Volatile_Timigns = [max(1, x // 10000000) for x in Doublets_Split_Volatile]
    Doublets_Split_NonVolatile_Timings = [max(1, x // 10000000) for x in Doublets_Split_NonVolatile]
    PSQL_NonTransaction_Timings = [max(1, x // 10000000) for x in PSQL_NonTransaction]
    PSQL_Transaction_Timings = [max(1, x // 10000000) for x in PSQL_Transaction]

    y = np.arange(len(labels))
    width = 0.1

    fig, ax = plt.subplots(figsize=(12, 8))

    ax.barh(y - 2 * width, Doublets_United_Volatile_Timings, width,
            label='Doublets United Volatile', color='salmon')
    ax.barh(y - width, Doublets_United_NonVolatile_Timings, width,
            label='Doublets United NonVolatile', color='red')
    ax.barh(y, Doublets_Split_Volatile_Timigns, width,
            label='Doublets Split Volatile', color='lightgreen')
    ax.barh(y + width, Doublets_Split_NonVolatile_Timings, width,
            label='Doublets Split NonVolatile', color='green')
    ax.barh(y + 2 * width, PSQL_NonTransaction_Timings, width,
            label='PSQL NonTransaction', color='lightblue')
    ax.barh(y + 3 * width, PSQL_Transaction_Timings, width,
            label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Scaled to Pixels')
    ax.set_title('Benchmark comparison for Doublets and PostgreSQL (Rust)')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()

    fig.tight_layout()
    plt.savefig("bench_rust.png")
    plt.close(fig)


def bench2():
    y = np.arange(len(labels))
    width = 0.1
    fig, ax = plt.subplots(figsize=(12, 8))

    ax.barh(y - 2 * width, Doublets_United_Volatile, width,
            label='Doublets United Volatile', color='salmon')
    ax.barh(y - width, Doublets_United_NonVolatile, width,
            label='Doublets United NonVolatile', color='red')
    ax.barh(y, Doublets_Split_Volatile, width,
            label='Doublets Split Volatile', color='lightgreen')
    ax.barh(y + width, Doublets_Split_NonVolatile, width,
            label='Doublets Split NonVolatile', color='green')
    ax.barh(y + 2 * width, PSQL_NonTransaction, width,
            label='PSQL NonTransaction', color='lightblue')
    ax.barh(y + 3 * width, PSQL_Transaction, width,
            label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Logarithmic Scale')
    ax.set_title('Benchmark comparison for Doublets and PostgreSQL (Rust)')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()

    ax.set_xscale('log')
    fig.tight_layout()
    plt.savefig("bench_rust_log_scale.png")
    plt.close(fig)


def print_results_table():
    # Print a header for the table of raw results
    header = "{:<20} {:<30} {:<30} {:<30} {:<30} {:<30} {:<30}".format(
        "Label", 
        "Doublets United Volatile", 
        "Doublets United NonVolatile", 
        "Doublets Split Volatile", 
        "Doublets Split NonVolatile", 
        "PSQL NonTransaction", 
        "PSQL Transaction"
    )
    print(header)
    print("-" * len(header))
    # Print each row with the corresponding raw data for each benchmark
    for i, label in enumerate(labels):
        du_vol = Doublets_United_Volatile[i] if i < len(Doublets_United_Volatile) else 'N/A'
        du_nonvol = Doublets_United_NonVolatile[i] if i < len(Doublets_United_NonVolatile) else 'N/A'
        ds_vol = Doublets_Split_Volatile[i] if i < len(Doublets_Split_Volatile) else 'N/A'
        ds_nonvol = Doublets_Split_NonVolatile[i] if i < len(Doublets_Split_NonVolatile) else 'N/A'
        psql_non = PSQL_NonTransaction[i] if i < len(PSQL_NonTransaction) else 'N/A'
        psql_trans = PSQL_Transaction[i] if i < len(PSQL_Transaction) else 'N/A'
        row = "{:<20} {:<30} {:<30} {:<30} {:<30} {:<30} {:<30}".format(
            label, du_vol, du_nonvol, ds_vol, ds_nonvol, psql_non, psql_trans
        )
        print(row)

print_results_table()
bench1()
bench2()
