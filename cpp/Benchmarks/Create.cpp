static void BM_PSQLCreateLinksWithoutTransaction(benchmark::State& state) {
    using namespace PostgreSQL;
    using namespace SetupTeardown;
    Client<std::uint64_t> table {options};
    SetupPSQL(table);
    for (auto _: state) {
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            CreatePoint(table);
        }
        state.PauseTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            Delete(table, {i, i, i});
        }
        state.ResumeTiming();
    }
    TeardownPSQL(table);
}

static void BM_PSQLCreateLinksWithTransaction(benchmark::State& state) {
    using namespace PostgreSQL;
    using namespace SetupTeardown;
    {
        Transaction<std::uint64_t> transaction {options};
        SetupPSQL(transaction);
    }
    for (auto _: state) {
        {
            state.PauseTiming();
            Transaction<std::uint64_t> transaction {options};
            state.ResumeTiming();
            for (std::uint64_t i {}; i < state.range(0); ++i) {
                CreatePoint(transaction);
            }
        }
        state.PauseTiming();
        {
            Transaction<std::uint64_t> transaction {options};
            for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
                Delete(transaction, {i, i, i});
            }
        }
        state.ResumeTiming();
    }
    Transaction<std::uint64_t> transaction {options};
    TeardownPSQL(transaction);
}

static void BM_DoubletsUnitedCreateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace SetupTeardown;
    std::filesystem::path path {"united.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage {FileMappedResizableDirectMemory{path.string()}};
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            storage.Delete({i, i, i}, handler);
        }
        state.ResumeTiming();
    }
    TeardownDoublets(storage);
}

static void BM_DoubletsUnitedCreateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(memory)};
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            storage.Delete({i, i, i}, handler);
        }
        state.ResumeTiming();
    }
    TeardownDoublets(storage);
}

static void BM_DoubletsSplitCreateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace Platform::Collections;
    using namespace SetupTeardown;
    std::filesystem::path split_data {"split_data.links"}, split_index {"split_index.links"};
    SplitMemoryLinks<LinksOptions<std::uint64_t>> storage {
        FileMappedResizableDirectMemory{split_data.string()},
        FileMappedResizableDirectMemory{split_index.string()}
    };
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            storage.Delete({i, i, i}, handler);
        }
        state.ResumeTiming();
    }
    TeardownDoublets(storage);
}

static void BM_DoubletsSplitCreateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory data {}, index {};
    SplitMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(data), std::move(index)};
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            storage.Delete({i, i, i}, handler);
        }
        state.ResumeTiming();
    }
    TeardownDoublets(storage);
}

BENCHMARK(BM_PSQLCreateLinksWithoutTransaction)->Name("BM_PSQL/Create/NonTransaction")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_PSQLCreateLinksWithTransaction)->Name("BM_PSQL/Create/Transaction")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsUnitedCreateLinksFile)->Name("BM_Doublets/United/Create/NonVolatile")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsUnitedCreateLinksRAM)->Name("BM_Doublets/United/Create/Volatile")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsSplitCreateLinksFile)->Name("BM_Doublets/Split/Create/NonVolatile")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsSplitCreateLinksRAM)->Name("BM_Doublets/Split/Create/Volatile")->Arg(1000)->MinWarmUpTime(10);
