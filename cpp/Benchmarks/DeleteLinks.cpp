static void BM_PSQLDeleteLinksWithoutTransaction(benchmark::State& state) {
    using namespace std::chrono;
    Client<std::uint64_t> table(options);
    //Use only one instance of connection to db
    auto setup = [&table, &state] {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.CreatePoint();
        }
    };
    for (auto _: state) {
        auto start = high_resolution_clock::now();
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Delete({i});
        }
        auto stop = high_resolution_clock::now();
        auto elapsed_time = duration_cast<duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
        setup();
    }
}

static void BM_PSQLDeleteLinksWithTransaction(benchmark::State& state) {
    using namespace std::chrono;
    time_point<system_clock, nanoseconds> start;
    for (auto _: state) {
        {
            Transaction<std::uint64_t> transaction(options);
            start = high_resolution_clock::now();
            for (std::uint64_t i = 1; i <= state.range(0); ++i) {
                transaction.Delete({i});
            }
        }
        auto stop = high_resolution_clock::now();
        auto elapsed_time = duration_cast<duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
        //Previous connection is closed
        internal::SetupPSQL(state);
    }
}

static void BM_DoubletsDeleteLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace std::chrono;
    std::filesystem::path path("db.links");
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    //Use only one instance of mapped file
    auto setup = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    auto any = storage.Constants.Any;
    for (auto _: state) {
        auto start = high_resolution_clock::now();
        for (std::uint64_t i = state.range(0); i != storage.Constants.Null; --i) {
            storage.Delete({i, any, any}, handler);
        }
        auto stop = high_resolution_clock::now();
        auto elapsed_time = duration_cast<duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
        setup();
    }
}

static void BM_DoubletsDeleteLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace std::chrono;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    //setup
    auto setup = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    setup();
    //Benchmark
    for (auto _: state) {
        auto start = high_resolution_clock::now();
        for (std::uint64_t i = state.range(0); i != storage.Constants.Null; --i) {
            storage.Delete({i, any, any}, handler);
        }
        auto stop = high_resolution_clock::now();
        auto elapsed_time = duration_cast<duration<std::double_t>>(stop-start).count();
        state.SetIterationTime(elapsed_time);
        //Another setup after iteration
        setup();
    }
    //Teardown
    DeleteAll(storage);
}

BENCHMARK(BM_PSQLDeleteLinksWithoutTransaction)->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL)->UseManualTime();
BENCHMARK(BM_PSQLDeleteLinksWithTransaction)->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL)->UseManualTime();
BENCHMARK(BM_DoubletsDeleteLinksRAM)->Arg(1000)->UseManualTime();
BENCHMARK(BM_DoubletsDeleteLinksFile)->Arg(1000)->Setup(internal::SetupDoubletsFile)->Teardown(internal::TeardownDoubletsFile)->UseManualTime();
