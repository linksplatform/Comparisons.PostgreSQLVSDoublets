import re
import logging
import matplotlib.pyplot as plt
import numpy as np

# Enable detailed tracing. Set to False to disable verbose output.
DEBUG = True
logging.basicConfig(level=logging.INFO if DEBUG else logging.WARNING,
                    format="%(message)s")

data = open('out.txt').read()
if DEBUG:
    logging.info("Loaded out.txt, length: %d characters", len(data))

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
    if DEBUG:
        logging.info("Pattern %s matched %d entries", pattern, len(matches))
    for match in matches:
        if match[0] == 'PSQL':
            _category, transaction, time = match
            if DEBUG:
                logging.info("PSQL - %s: %s ns", transaction, time)
            if transaction == "Transaction":
                PSQL_Transaction.append(int(time))
            else:
                PSQL_NonTransaction.append(int(time))
        else:
            _category, trees, storage, time = match
            if DEBUG:
                logging.info("Doublets %s %s: %s ns", trees, storage, time)
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

if DEBUG:
    logging.info("\nFinal lists (after parsing):")
    logging.info("PSQL_Transaction: %s", PSQL_Transaction)
    logging.info("PSQL_NonTransaction: %s", PSQL_NonTransaction)
    logging.info("Doublets_United_Volatile: %s", Doublets_United_Volatile)
    logging.info("Doublets_United_NonVolatile: %s", Doublets_United_NonVolatile)
    logging.info("Doublets_Split_Volatile: %s", Doublets_Split_Volatile)
    logging.info("Doublets_Split_NonVolatile: %s", Doublets_Split_NonVolatile)

labels = ['Create', 'Delete', 'Each Identity', 'Each Concrete', 'Each Outgoing', 'Each Incoming', 'Each All', 'Update']

# ─────────────────────────────────────────────────────────────────────────────
# Plots
# ─────────────────────────────────────────────────────────────────────────────
def ensure_min_visible(arr, min_val):
    """Ensure non-zero values are at least min_val for visibility on graph."""
    return [max(v, min_val) if v > 0 else 0 for v in arr]

def bench1():
    """Horizontal bars – raw values (pixel scale)."""
    y = np.arange(len(labels))
    width = 0.1
    fig, ax = plt.subplots(figsize=(12, 8))

    # Calculate maximum value across all data series to determine scale
    all_values = (Doublets_United_Volatile + Doublets_United_NonVolatile +
                  Doublets_Split_Volatile + Doublets_Split_NonVolatile +
                  PSQL_NonTransaction + PSQL_Transaction)
    max_val = max(all_values) if all_values else 1

    # Minimum visible bar width: ~0.5% of max value ensures at least 2 pixels
    # on typical 12-inch wide figure at 100 DPI (~900px plot area)
    min_visible = max_val * 0.005
    if DEBUG:
        logging.info("bench1: max_val=%d, min_visible=%d", max_val, min_visible)

    # Apply minimum visibility to all data series
    du_volatile_vis = ensure_min_visible(Doublets_United_Volatile, min_visible)
    du_nonvolatile_vis = ensure_min_visible(Doublets_United_NonVolatile, min_visible)
    ds_volatile_vis = ensure_min_visible(Doublets_Split_Volatile, min_visible)
    ds_nonvolatile_vis = ensure_min_visible(Doublets_Split_NonVolatile, min_visible)
    psql_non_vis = ensure_min_visible(PSQL_NonTransaction, min_visible)
    psql_trans_vis = ensure_min_visible(PSQL_Transaction, min_visible)

    ax.barh(y - 2*width, du_volatile_vis, width, label='Doublets United Volatile', color='salmon')
    ax.barh(y - width, du_nonvolatile_vis, width, label='Doublets United NonVolatile', color='red')
    ax.barh(y, ds_volatile_vis, width, label='Doublets Split Volatile', color='lightgreen')
    ax.barh(y + width, ds_nonvolatile_vis, width, label='Doublets Split NonVolatile', color='green')
    ax.barh(y + 2*width, psql_non_vis, width, label='PSQL NonTransaction', color='lightblue')
    ax.barh(y + 3*width, psql_trans_vis, width, label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns)')
    ax.set_title('Benchmark Comparison: PostgreSQL vs Doublets (C++)')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()

    fig.tight_layout()
    plt.savefig("bench_cpp.png")
    plt.close(fig)
    if DEBUG: logging.info("bench_cpp.png saved.")

def bench2():
    """Horizontal bars – raw values on a log scale."""
    y = np.arange(len(labels))
    width = 0.1
    fig, ax = plt.subplots(figsize=(12, 8))

    ax.barh(y - 2*width, Doublets_United_Volatile, width, label='Doublets United Volatile', color='salmon')
    ax.barh(y - width, Doublets_United_NonVolatile, width, label='Doublets United NonVolatile', color='red')
    ax.barh(y, Doublets_Split_Volatile, width, label='Doublets Split Volatile', color='lightgreen')
    ax.barh(y + width, Doublets_Split_NonVolatile, width, label='Doublets Split NonVolatile', color='green')
    ax.barh(y + 2*width, PSQL_NonTransaction, width, label='PSQL NonTransaction', color='lightblue')
    ax.barh(y + 3*width, PSQL_Transaction, width, label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) – log scale')
    ax.set_title('Benchmark Comparison: PostgreSQL vs Doublets (C++)')
    ax.set_yticks(y)
    ax.set_yticklabels(labels)
    ax.legend()
    ax.set_xscale('log')

    fig.tight_layout()
    plt.savefig("bench_cpp_log_scale.png")
    plt.close(fig)
    if DEBUG: logging.info("bench_cpp_log_scale.png saved.")

bench1()
bench2()
