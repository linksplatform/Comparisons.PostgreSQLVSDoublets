import re
import matplotlib.pyplot as plt
import numpy as np

data = open('cmake-build-release/out.txt').read()

patterns = [
    r"BM_(PSQL)/\w+\/?\w+?/(\w+)/\d+/min_warmup_time:20\.000\s+(\d+)\sns\s+\d+\sns\s+\d+",
    r"BM_(Doublets)/(\w+)/\w+\/?\w+?/(\w+)/\d+/min_warmup_time:20\.000\s+(\d+)\sns\s+\d+\sns\s+\d+",
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
        if match[0] == 'PSQL':
            _category, transaction, time = match
            if transaction == "Transaction":
                PSQL_Transaction.append(int(time))
            else:
                PSQL_NonTransaction.append(int(time))
        else:
            _category, trees, storage, time = match
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

Doublets_United_Volatile = [max(1, x // 10000000) for x in Doublets_United_Volatile]
Doublets_United_NonVolatile = [max(1, x // 10000000) for x in Doublets_United_NonVolatile]
Doublets_Split_Volatile = [max(1, x // 10000000) for x in Doublets_Split_Volatile]
Doublets_Split_NonVolatile = [max(1, x // 10000000) for x in Doublets_Split_NonVolatile]
PSQL_NonTransaction = [max(1, x // 10000000) for x in PSQL_NonTransaction]
PSQL_Transaction = [max(1, x // 10000000) for x in PSQL_Transaction]

labels = ['Create', 'Delete', 'Each Identity', 'Each Concrete', 'Each Outgoing', 'Each Incoming', 'Each All', 'Update']
def bench1():
    y = np.arange(len(labels))

    width = 0.1

    fig, ax = plt.subplots(figsize=(12, 8))


    rects1 = ax.barh(y - 2*width, Doublets_United_Volatile, width, label='Doublets United Volatile', color='salmon')
    rects2 = ax.barh(y - width, Doublets_United_NonVolatile, width, label='Doublets United NonVolatile', color='red')

    rects3 = ax.barh(y, Doublets_Split_Volatile, width, label='Doublets Split Volatile', color='lightgreen')
    rects4 = ax.barh(y + width, Doublets_Split_NonVolatile, width, label='Doublets Split NonVolatile', color='green')

    rects5 = ax.barh(y + 2*width, PSQL_NonTransaction, width, label='PSQL NonTransaction', color='lightblue')
    rects6 = ax.barh(y + 3*width, PSQL_Transaction, width, label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Scaled to Pixels')
    ax.set_title('Benchmark comparison for Doublets and BM_PSQL')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()

    fig.tight_layout()

    plt.savefig("Bench1.png")
    plt.close(fig)

def bench2():
    y = np.arange(len(labels))

    width = 0.1
    fig, ax = plt.subplots(figsize=(12, 8))

    rects1 = ax.barh(y - 2*width, Doublets_United_Volatile, width, label='Doublets United Volatile', color='salmon')
    rects2 = ax.barh(y - width, Doublets_United_NonVolatile, width, label='Doublets United NonVolatile', color='red')

    rects3 = ax.barh(y, Doublets_Split_Volatile, width, label='Doublets Split Volatile', color='lightgreen')
    rects4 = ax.barh(y + width, Doublets_Split_NonVolatile, width, label='Doublets Split NonVolatile', color='green')

    rects5 = ax.barh(y + 2*width, PSQL_NonTransaction, width, label='PSQL NonTransaction', color='lightblue')
    rects6 = ax.barh(y + 3*width, PSQL_Transaction, width, label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Logarithmic Scale')
    ax.set_title('Benchmark comparison for Doublets and BM_PSQL')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()

    ax.set_xscale('log')

    fig.tight_layout()

    plt.savefig("Bench2.png")
    plt.close(fig)

bench1()
bench2()
