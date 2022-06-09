#include <benchmark/benchmark.h>
#include "LinksPSQL.h"

static void BM_CreateThousandLinks(benchmark::State& state) {
    const std::string opts = "";
    for (auto _: state) {
        {
            LinksPSQL<std::uint64_t> table(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector substitution{i, i};
                table.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            LinksPSQL<std::uint64_t> links(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector restriction {i, i};
                links.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}

BENCHMARK(BM_CreateThousandLinks)->Arg(1000);
BENCHMARK_MAIN();
