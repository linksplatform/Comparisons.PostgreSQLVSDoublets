static void BM_PSQLUpdateLinksWithoutTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    Client<LinksOptions<std::uint64_t>> table {options};
    Setup(table);
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            Update(table, {i}, {0, 0, 0});
            Update(table, {i}, {i, i, i});
        }
    }
    Teardown(table);
}

static void BM_PSQLUpdateLinksWithTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    {
        Transaction<LinksOptions<std::uint64_t>> table {options};
        Setup(table);
    }
    for (auto _: state) {
        state.PauseTiming();
        Transaction<LinksOptions<std::uint64_t>> table {options};
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            Update(table, {i}, {0, 0, 0});
            Update(table, {i}, {i, i, i});
        }
    }
    Transaction<LinksOptions<std::uint64_t>> table {options};
    Teardown(table);
}

static void BM_DoubletsUnitedUpdateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    std::filesystem::path path {"united.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage {FileMappedResizableDirectMemory{path.string()}};
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            Update(storage, {i}, {0, 0, 0});
            Update(storage, {i}, {i, i, i});
        }
    }
    Teardown(storage);
}

static void BM_DoubletsUnitedUpdateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(memory)};
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            Update(storage, {i}, {0, 0, 0});
            Update(storage, {i}, {i, i, i});
        }
    }
    Teardown(storage);
}

static void BM_DoubletsSplitUpdateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    std::filesystem::path split_data {"split_data.links"}, split_index {"split_index.links"};
    SplitMemoryLinks<LinksOptions<std::uint64_t>> storage {
        FileMappedResizableDirectMemory{split_data.string()},
        FileMappedResizableDirectMemory{split_index.string()}
    };
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            Update(storage, {i}, {0, 0, 0});
            Update(storage, {i}, {i, i, i});
        }
    }
    Teardown(storage);
}

static void BM_DoubletsSplitUpdateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory data {}, index {};
    SplitMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(data), std::move(index)};
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            Update(storage, {i}, {0, 0, 0});
            Update(storage, {i}, {i, i, i});
        }
    }
    Teardown(storage);
}

BENCHMARK(BM_PSQLUpdateLinksWithoutTransaction)->Name("BM_PSQL/Update/NonTransaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_PSQLUpdateLinksWithTransaction)->Name("BM_PSQL/Update/Transaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedUpdateLinksFile)->Name("BM_Doublets/United/Update/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedUpdateLinksRAM)->Name("BM_Doublets/United/Update/Volatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitUpdateLinksFile)->Name("BM_Doublets/Split/Update/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitUpdateLinksRAM)->Name("BM_Doublets/Split/Update/Volatile")->Arg(1000)->MinWarmUpTime(20);
