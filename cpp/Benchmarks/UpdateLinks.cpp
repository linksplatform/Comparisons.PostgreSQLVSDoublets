static void BM_PSQLUpdateLinksWithoutTransaction(benchmark::State& state) {
    Client<std::uint64_t> table(options);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Update({i}, {i+1, i+2});
        }
    }
}

static void BM_PSQLUpdateLinksWithTransaction(benchmark::State& state) {
    std::chrono::time_point<std::chrono::system_clock, std::chrono::nanoseconds> start;
    for (auto _: state) {
        {
            Transaction<std::uint64_t> table (options);
            start = std::chrono::high_resolution_clock::now();
            for (std::uint64_t i = 1; i <= state.range(0); ++i) {
                table.Update({i}, {i+1, i+2});
            }
        }
        auto stop = std::chrono::high_resolution_clock::now();
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
    }
}

static void BM_DoubletsUpdateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    std::filesystem::path path {"db.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Update({i}, {i+1, i+2}, handler);
        }
    }
}

static void BM_DoubletsUpdateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    //Setup
    for (std::size_t i{}; i < state.range(0); ++i) {
        CreatePoint(storage);
    }
    //Benchmark
    for (auto _: state) {
        auto start = std::chrono::high_resolution_clock::now();
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Update({i}, {i+1, i+2}, handler);
        }
        auto stop = std::chrono::high_resolution_clock::now();
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
    }
    //Teardown
    DeleteAll(storage);
}

BENCHMARK(BM_PSQLUpdateLinksWithoutTransaction)->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLUpdateLinksWithTransaction)->Arg(1000)->UseManualTime()->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsUpdateLinksRAM)->Arg(1000)->UseManualTime();
BENCHMARK(BM_DoubletsUpdateLinksFile)->Arg(1000)->Setup(internal::SetupDoubletsFile)->Teardown(internal::TeardownDoubletsFile);
