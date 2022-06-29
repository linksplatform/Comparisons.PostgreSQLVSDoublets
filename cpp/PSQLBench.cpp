#include <benchmark/benchmark.h>

#include "Client.h"
#include "Transaction.h"

static void BM_CreateThousandLinksWithoutTransaction(benchmark::State& state) {
    const std::string opts = "";
    for (auto _ : state) {
        {
            Client<std::uint64_t> table(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector substitution{i, i};
                table.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            Client<std::uint64_t> links(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector restriction {i, i};
                links.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}
static void BM_CreateThousandLinksWithTransaction(benchmark::State& state) {
    const std::string opts = "";
    for (auto _ : state) {
        {
            Transaction<std::uint64_t> transaction(opts);
            for (std::uint64_t i {1}; i <= state.range(0); ++i) {
                std::vector substitution {i, i};
                transaction.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            Transaction<std::uint64_t> transaction(opts);
            for (std::uint64_t i {1}; i <= state.range(0); ++i) {
                std::vector restriction {i, i};
                transaction.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}

BENCHMARK(BM_CreateThousandLinksWithoutTransaction)->Arg(1000);
BENCHMARK(BM_CreateThousandLinksWithTransaction)->Arg(1000);
BENCHMARK_MAIN();
