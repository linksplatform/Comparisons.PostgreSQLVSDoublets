import re
import matplotlib.pyplot as plt
import numpy as np

# Read the complete file
data = open("out.txt").read()

# Use two regex patterns to extract benchmarks.
# The first captures PostgreSQL tests and the second captures Doublets tests.
patterns = [
    r"test\s+(\w+)/(PSQL)_(\w+)\s+\.\.\.\s+bench:\s+(\d+)\s+ns/iter\s+\(\+/-\s+\d+\)",
    r"test\s+(\w+)/(Doublets)_(\w+)_(\w+)\s+\.\.\.\s+bench:\s+(\d+)\s+ns/iter\s+\(\+/-\s+\d+\)"
]

# Instead of using lists, we use dictionaries mapping operation names to values.
PSQL_Transaction = {}
PSQL_NonTransaction = {}
Doublets_United_Volatile = {}
Doublets_United_NonVolatile = {}
Doublets_Split_Volatile = {}
Doublets_Split_NonVolatile = {}

# Process each regex pattern
for pattern in patterns:
    matches = re.findall(pattern, data)
    for match in matches:
        op = match[0]  # the operation name (e.g., Create, Update, etc.)
        if match[1] == 'PSQL':
            # For PostgreSQL: (operation, 'PSQL', transaction, time)
            transaction = match[2]
            time_val = int(match[3])
            if transaction == "Transaction":
                PSQL_Transaction[op] = time_val
            else:
                PSQL_NonTransaction[op] = time_val
        else:
            # For Doublets: (operation, 'Doublets', trees, storage, time)
            trees = match[2]
            storage = match[3]
            time_val = int(match[4])
            if trees == 'United':
                if storage == 'Volatile':
                    Doublets_United_Volatile[op] = time_val
                else:
                    Doublets_United_NonVolatile[op] = time_val
            else:
                if storage == 'Volatile':
                    Doublets_Split_Volatile[op] = time_val
                else:
                    Doublets_Split_NonVolatile[op] = time_val

# Set the unified order.
# First write operations then read operations exactly as in the sample table.
ordered_ops = [
    "Create", "Update", "Delete",    # Write operations
    "Each All", "Each Identity", "Each Concrete", "Each Outgoing", "Each Incoming"  # Read operations
]

# Prepare arrays for plotting by reassembling results in the desired order.
def get_series(data_dict):
    # If an operation is missing in the dictionary, default to 0.
    return [data_dict.get(op, 0) for op in ordered_ops]

du_volatile_arr = get_series(Doublets_United_Volatile)
du_nonvolatile_arr = get_series(Doublets_United_NonVolatile)
ds_volatile_arr = get_series(Doublets_Split_Volatile)
ds_nonvolatile_arr = get_series(Doublets_Split_NonVolatile)
psql_non_arr = get_series(PSQL_NonTransaction)
psql_trans_arr = get_series(PSQL_Transaction)

#########################################
# Print the Markdown Table to Console
#########################################
def print_results_markdown():
    # Define the header in Markdown format.
    header = (
        "| Operation     | Doublets United Volatile | Doublets United NonVolatile | "
        "Doublets Split Volatile | Doublets Split NonVolatile | PSQL NonTransaction | PSQL Transaction |\n"
        "|---------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------------|------------------|"
    )
    lines = [header]
    
    # For each operation row, compute the minimum PostgreSQL time
    # and annotate each Doublets result with the relative speed-up.
    for i, op in enumerate(ordered_ops):
        # Get PostgreSQL values. If one is missing, use the other.
        psql_val1 = psql_non_arr[i] if psql_non_arr[i] != 0 else float('inf')
        psql_val2 = psql_trans_arr[i] if psql_trans_arr[i] != 0 else float('inf')
        min_psql = min(psql_val1, psql_val2)
        
        def annotate(doublets_val):
            if doublets_val == 0:
                return "N/A"
            # Compute the factor (using the fastest PostgreSQL result).
            factor = min_psql / doublets_val
            return f"{doublets_val} ({factor:.1f}+ times faster)"
        
        du_vol_str = annotate(du_volatile_arr[i])
        du_nonvol_str = annotate(du_nonvolatile_arr[i])
        ds_vol_str = annotate(ds_volatile_arr[i])
        ds_nonvol_str = annotate(ds_nonvolatile_arr[i])
        
        # For PostgreSQL values, just show the raw number or N/A if missing.
        psql_non_str = str(psql_non_arr[i]) if psql_non_arr[i] != 0 else "N/A"
        psql_trans_str = str(psql_trans_arr[i]) if psql_trans_arr[i] != 0 else "N/A"
        
        row = (
            f"| {op:<13} | {du_vol_str:<24} | {du_nonvol_str:<27} | "
            f"{ds_vol_str:<23} | {ds_nonvol_str:<26} | {psql_non_str:<19} | {psql_trans_str:<16} |"
        )
        lines.append(row)
    
    table_md = "\n".join(lines)
    print(table_md)

#########################################
# Plotting Functions with Unified Order
#########################################
def bench1():
    """
    This function plots horizontal bar charts with scaled times.
    Scaled by dividing each raw nanosecond (ns) value by 10,000,000.
    """
    # Scale the Doublets and PostgreSQL arrays.
    scale_factor = 10000000
    du_volatile_scaled = [max(1, x // scale_factor) for x in du_volatile_arr]
    du_nonvolatile_scaled = [max(1, x // scale_factor) for x in du_nonvolatile_arr]
    ds_volatile_scaled = [max(1, x // scale_factor) for x in ds_volatile_arr]
    ds_nonvolatile_scaled = [max(1, x // scale_factor) for x in ds_nonvolatile_arr]
    psql_non_scaled = [max(1, x // scale_factor) for x in psql_non_arr]
    psql_trans_scaled = [max(1, x // scale_factor) for x in psql_trans_arr]

    y = np.arange(len(ordered_ops))
    width = 0.1

    fig, ax = plt.subplots(figsize=(12, 8))

    ax.barh(y - 2 * width, du_volatile_scaled, width,
            label='Doublets United Volatile', color='salmon')
    ax.barh(y - width, du_nonvolatile_scaled, width,
            label='Doublets United NonVolatile', color='red')
    ax.barh(y, ds_volatile_scaled, width,
            label='Doublets Split Volatile', color='lightgreen')
    ax.barh(y + width, ds_nonvolatile_scaled, width,
            label='Doublets Split NonVolatile', color='green')
    ax.barh(y + 2 * width, psql_non_scaled, width,
            label='PSQL NonTransaction', color='lightblue')
    ax.barh(y + 3 * width, psql_trans_scaled, width,
            label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Scaled to Pixels')
    ax.set_title('Benchmark Comparison for Doublets and PostgreSQL (Rust)')
    ax.set_yticks(y)
    ax.set_yticklabels(ordered_ops)
    ax.legend()

    fig.tight_layout()
    plt.savefig("bench_rust.png")
    plt.close(fig)

def bench2():
    """
    This function plots horizontal bar charts using raw nanosecond values on a logarithmic scale.
    """
    y = np.arange(len(ordered_ops))
    width = 0.1
    fig, ax = plt.subplots(figsize=(12, 8))

    ax.barh(y - 2 * width, du_volatile_arr, width,
            label='Doublets United Volatile', color='salmon')
    ax.barh(y - width, du_nonvolatile_arr, width,
            label='Doublets United NonVolatile', color='red')
    ax.barh(y, ds_volatile_arr, width,
            label='Doublets Split Volatile', color='lightgreen')
    ax.barh(y + width, ds_nonvolatile_arr, width,
            label='Doublets Split NonVolatile', color='green')
    ax.barh(y + 2 * width, psql_non_arr, width,
            label='PSQL NonTransaction', color='lightblue')
    ax.barh(y + 3 * width, psql_trans_arr, width,
            label='PSQL Transaction', color='blue')

    ax.set_xlabel('Time (ns) - Logarithmic Scale')
    ax.set_title('Benchmark Comparison for Doublets and PostgreSQL (Rust)')
    ax.set_yticks(y)
    ax.set_yticklabels(ordered_ops)
    ax.legend()

    ax.set_xscale('log')
    fig.tight_layout()
    plt.savefig("bench_rust_log_scale.png")
    plt.close(fig)

#########################################
# Main section: print table and generate images
#########################################
print_results_markdown()
bench1()
bench2()
