namespace Platform::Data::Doublets::Benchmarks
{
    using namespace Platform::Data::Doublets;

    const std::string OPTS = "";
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
}
