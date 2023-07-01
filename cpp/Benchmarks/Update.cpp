static void BM_PSQLUpdateLinksWithoutTransaction(benchmark::State& state) {
    Client<std::uint64_t> table(options);
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            table.Update({i}, {0, 0});
            table.Update({i}, {i, i});
        }
    }
}

static void BM_PSQLUpdateLinksWithTransaction(benchmark::State& state) {
    for (auto _: state) {
        state.PauseTiming();
        Transaction<std::uint64_t> table (options);
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            table.Update({i}, {0, 0});
            table.Update({i}, {i, i});
        }
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
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Update({i}, {0, 0}, handler);
            storage.Update({i}, {i, i}, handler);
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
    auto handler_update = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    for (std::size_t i {}; i < BACKGROUND_LINKS; ++i) {
        CreatePoint(storage);
    }
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Update({i}, {0, 0}, handler_update);
            storage.Update({i}, {i, i}, handler_update);
        }
    }
}

BENCHMARK(BM_PSQLUpdateLinksWithoutTransaction)->Name("BM_PSQL/Update/NonTransaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLUpdateLinksWithTransaction)->Name("BM_PSQL/Update/Transaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsUpdateLinksRAM)->Name("BM_Doublets/Update/Volatile")->Arg(1000);
BENCHMARK(BM_DoubletsUpdateLinksFile)->Name("BM_Doublets/Update/NonVolatile")->Arg(1000)->Setup(internal::SetupDoublets)->Teardown(internal::TeardownDoublets);
