static void BM_PSQLCreateLinksWithoutTransaction(benchmark::State& state) {
    Client<std::uint64_t> table(options);
    for (auto _: state) {
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            table.CreatePoint();
        }
        state.PauseTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            table.Delete({i});
        }
        state.ResumeTiming();
    }
}

static void BM_PSQLCreateLinksWithTransaction(benchmark::State& state) {
    for (auto _: state) {
        {
            state.PauseTiming();
            Transaction<std::uint64_t> transaction(options);
            state.ResumeTiming();
            for (std::uint64_t i {}; i < state.range(0); ++i) {
                transaction.CreatePoint();
            }
        }
        state.PauseTiming();
        {
            Transaction<std::uint64_t> transaction(options);
            for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
                transaction.Delete({i});
            }
        }
        state.ResumeTiming();
    }
}

static void BM_DoubletsCreateLinksFile(benchmark::State& state) {
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
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        for (std::size_t i = storage.Count({}); i > BACKGROUND_LINKS; i = storage.Count({})) {
            storage.Delete({i, i, i}, handler);
        }
        state.ResumeTiming();
    }
}

static void BM_DoubletsCreateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    for (std::size_t i{}; i < BACKGROUND_LINKS; ++i) {
        CreatePoint(storage);
    }
    for (auto _: state) {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        for (std::size_t i = storage.Count({}); i > BACKGROUND_LINKS; i = storage.Count({})) {
            storage.Delete({i, i, i}, handler);
        }
        state.ResumeTiming();
    }
}


BENCHMARK(BM_PSQLCreateLinksWithoutTransaction)->Name("BM_PSQL/Create/NoTransaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLCreateLinksWithTransaction)->Name("BM_PSQL/Create/Transaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsCreateLinksRAM)->Name("BM_Doublets/Create/Volatile")->Arg(1000);
BENCHMARK(BM_DoubletsCreateLinksFile)->Name("BM_Doublets/Create/NonVolatile")->Arg(1000)->Setup(internal::SetupDoublets)->Teardown(internal::TeardownDoublets);
