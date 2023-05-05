static void BM_PSQLEachLinksWithoutTransaction(benchmark::State& state) {
    using namespace Platform::Data;
    Client<std::uint64_t> table(options);
    auto any = LinksConstants<std::uint64_t>().Any;
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Each({i, any, any});
        }
    }
}

static void BM_PSQLEachLinksWithTransaction(benchmark::State& state) {
    using namespace std::chrono;
    time_point<system_clock, nanoseconds> start;
    auto any = Platform::Data::LinksConstants<std::uint64_t>().Any;
    for (auto _: state) {
        {
            Transaction<std::uint64_t> table (options);
            start = high_resolution_clock::now();
            for (std::uint64_t i = 1; i <= state.range(0); ++i) {
                table.Each({i, any, any});
            }
        }
        auto stop = high_resolution_clock::now();
        auto elapsed_time = duration_cast<duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
    }
}

static void BM_DoubletsEachLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    std::filesystem::path path {"db.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({i, any, any}, handler);
        }
    }
}

static void BM_DoubletsEachLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace std::chrono;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    //Setup
    for (std::size_t i{}; i < state.range(0); ++i) {
        CreatePoint(storage);
    }
    //Benchmark
    for (auto _: state) {
        auto start = high_resolution_clock::now();
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({i, any, any}, handler);
        }
        auto stop = high_resolution_clock::now();
        auto elapsed_time = duration_cast<duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
    }
    //Teardown
    DeleteAll(storage);
}

BENCHMARK(BM_PSQLEachLinksWithoutTransaction)->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLEachLinksWithTransaction)->Arg(1000)->UseManualTime()->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsEachLinksRAM)->Arg(1000)->UseManualTime();
BENCHMARK(BM_DoubletsEachLinksFile)->Arg(1000)->Setup(internal::SetupDoubletsFile)->Teardown(internal::TeardownDoubletsFile);
