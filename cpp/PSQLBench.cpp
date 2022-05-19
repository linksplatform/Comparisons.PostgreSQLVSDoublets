#include "LinksPSQL.h"
#include <benchmark/benchmark.h>

const std::string OPTS = "host=localhost user=mitron57 dbname=mitron57 password='mitron57' port=5432";
LinksPSQL<int> table(OPTS);

static void BM_CreateMillionLinks(benchmark::State& state)
{
    for(auto _ : state)
    {
        for(std::size_t i = 0; i<state.range(0); ++i)
        {
            std::vector sub{i+1, i+1};
            table.Create(sub);
        }
        table.Complete();
    }
}

BENCHMARK(BM_CreateMillionLinks)->Arg(1000000);
BENCHMARK_MAIN();
