static void BM_PSQLEachLinkWithoutTransaction(benchmark::State& state) {
    using namespace Platform::Data;
    Client<std::uint64_t> table(options);
    auto any = LinksConstants<std::uint64_t>().Any;
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Each({i, any, any});
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            table.Each({i, any, any});
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            table.Each({i, any, any});
        }
    }
}

static void BM_PSQLEachLinkWithTransaction(benchmark::State& state) {
    using namespace Platform::Data;
    auto any = LinksConstants<std::uint64_t>().Any;
    for (auto _: state) {
        state.PauseTiming();
        Transaction<std::uint64_t> table (options);
        state.ResumeTiming();
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Each({i, any, any});
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            table.Each({i, any, any});
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            table.Each({i, any, any});
        }
    }
}

static void BM_DoubletsEachLinkFile(benchmark::State& state) {
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
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            storage.Each({i, any, any}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Each({i, any, any}, handler);
        }
    }
}

static void BM_DoubletsEachLinkRAM(benchmark::State& state) {
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
    for (std::size_t i{}; i < BACKGROUND_LINKS; ++i) {
        CreatePoint(storage);
    }
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({i, any, any}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            storage.Each({i, any, any}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Each({i, any, any}, handler);
        }
    }
}

BENCHMARK(BM_PSQLEachLinkWithoutTransaction)->Name("BM_PSQL/Each/NoTransaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLEachLinkWithTransaction)->Name("BM_PSQL/Each/Transaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsEachLinkRAM)->Name("BM_Doublets/Each/Volatile")->Arg(1000);
BENCHMARK(BM_DoubletsEachLinkFile)->Name("BM_Doublets/Each/NonVolatile")->Arg(1000)->Setup(internal::SetupDoublets)->Teardown(internal::TeardownDoublets);
