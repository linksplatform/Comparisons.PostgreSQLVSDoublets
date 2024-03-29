static void BM_PSQLCreateLinksWithoutTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    Client<LinksOptions<std::uint64_t>> table {options};
    auto background = BACKGROUND_LINKS;
    Setup(table);
    for (auto _: state) {
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            CreatePoint(table);
        }
        state.PauseTiming();
        for (std::uint64_t i = background + state.range(0); i > background; --i) {
            Delete(table, {i, i, i});
        }
        background += state.range(0);
        state.ResumeTiming();
    }
    Teardown(table);
}

static void BM_PSQLCreateLinksWithTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    {
        Transaction<LinksOptions<std::uint64_t>> transaction {options};
        Setup(transaction);
    }
    auto background = BACKGROUND_LINKS;
    for (auto _: state) {
        {
            state.PauseTiming();
            Transaction<LinksOptions<std::uint64_t>> transaction {options};
            state.ResumeTiming();
            for (std::uint64_t i {}; i < state.range(0); ++i) {
                CreatePoint(transaction);
            }
        }
        state.PauseTiming();
        {
            Transaction<LinksOptions<std::uint64_t>> transaction {options};
            for (std::uint64_t i = background + state.range(0); i > background; --i) {
                Delete(transaction, {i, i, i});
            }
            background += state.range(0);
        }
        state.ResumeTiming();
    }
    Transaction<LinksOptions<std::uint64_t>> transaction {options};
    Teardown(transaction);
}

static void BM_DoubletsUnitedCreateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    std::filesystem::path path {"united.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage {FileMappedResizableDirectMemory{path.string()}};
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    Setup(storage);
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
    Teardown(storage);
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
    Setup(storage);
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
    Teardown(storage);
}

static void BM_DoubletsSplitCreateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    std::filesystem::path split_data {"split_data.links"}, split_index {"split_index.links"};
    SplitMemoryLinks<LinksOptions<std::uint64_t>> storage {
        FileMappedResizableDirectMemory{split_data.string()},
        FileMappedResizableDirectMemory{split_index.string()}
    };
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    Setup(storage);
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
    Teardown(storage);
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
    Setup(storage);
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
    Teardown(storage);
}

BENCHMARK(BM_PSQLCreateLinksWithoutTransaction)->Name("BM_PSQL/Create/NonTransaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_PSQLCreateLinksWithTransaction)->Name("BM_PSQL/Create/Transaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedCreateLinksFile)->Name("BM_Doublets/United/Create/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedCreateLinksRAM)->Name("BM_Doublets/United/Create/Volatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitCreateLinksFile)->Name("BM_Doublets/Split/Create/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitCreateLinksRAM)->Name("BM_Doublets/Split/Create/Volatile")->Arg(1000)->MinWarmUpTime(20);
