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
    Doublets_United_Volatile_Timings = [max(1, x // 10000000) for x in Doublets_United_Volatile]
    Doublets_United_NonVolatile_Timings = [max(1, x // 10000000) for x in Doublets_United_NonVolatile]
    Doublets_Split_Volatile_Timigns = [max(1, x // 10000000) for x in Doublets_Split_Volatile]
    Doublets_Split_NonVolatile_Timings = [max(1, x // 10000000) for x in Doublets_Split_NonVolatile]
    PSQL_NonTransaction_Timings = [max(1, x // 10000000) for x in PSQL_NonTransaction]
    PSQL_Transaction_Timings = [max(1, x // 10000000) for x in PSQL_Transaction]

    y = np.arange(len(labels))

    width = 0.1

    fig, ax = plt.subplots(figsize=(12, 8))

    rects1 = ax.barh(y - 2 * width, Doublets_United_Volatile_Timings, width, label='Doublets United Volatile',
                     color='salmon')
    rects2 = ax.barh(y - width, Doublets_United_NonVolatile_Timings, width, label='Doublets United NonVolatile',
                     color='red')

    rects3 = ax.barh(y, Doublets_Split_Volatile_Timigns, width, label='Doublets Split Volatile', color='lightgreen')
    rects4 = ax.barh(y + width, Doublets_Split_NonVolatile_Timings, width, label='Doublets Split NonVolatile',
                     color='green')

    rects5 = ax.barh(y + 2 * width, PSQL_NonTransaction_Timings, width, label='PSQL NonTransaction', color='lightblue')
    rects6 = ax.barh(y + 3 * width, PSQL_Transaction_Timings, width, label='PSQL Transaction', color='blue')

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

    rects1 = ax.barh(y - 2 * width, Doublets_United_Volatile, width, label='Doublets United Volatile', color='salmon')
    rects2 = ax.barh(y - width, Doublets_United_NonVolatile, width, label='Doublets United NonVolatile', color='red')

    rects3 = ax.barh(y, Doublets_Split_Volatile, width, label='Doublets Split Volatile', color='lightgreen')
    rects4 = ax.barh(y + width, Doublets_Split_NonVolatile, width, label='Doublets Split NonVolatile', color='green')

    rects5 = ax.barh(y + 2 * width, PSQL_NonTransaction, width, label='PSQL NonTransaction', color='lightblue')
    rects6 = ax.barh(y + 3 * width, PSQL_Transaction, width, label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Logarithmic Scale')
    ax.set_title('Benchmark comparison for Doublets and PostgreSQL (Rust)')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()

    ax.set_xscale('log')

    fig.tight_layout()

    plt.savefig("bench_rust_log_scale.png")
    plt.close(fig)


bench1()
bench2()
