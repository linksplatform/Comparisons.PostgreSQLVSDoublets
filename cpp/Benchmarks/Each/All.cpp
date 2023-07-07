static void BM_PSQLEachAllWithoutTransaction(benchmark::State& state) {
    using namespace PostgreSQL;
    using namespace SetupTeardown;
    Client<std::uint64_t> table {options};
    auto any = Platform::Data::LinksConstants<std::uint64_t>().Any;
    SetupPSQL(table);
    for (auto _: state) {
        Each(table, {any, any, any});
    }
    TeardownPSQL(table);
}

static void BM_PSQLEachAllWithTransaction(benchmark::State& state) {
    using namespace PostgreSQL;
    using namespace SetupTeardown;
    auto any = Platform::Data::LinksConstants<std::uint64_t>().Any;
    {
        Transaction<std::uint64_t> table {options};
        SetupPSQL(table);
    }
    for (auto _: state) {
        state.PauseTiming();
        Transaction<std::uint64_t> table {options};
        state.ResumeTiming();
        Each(table, {any, any, any});
    }
    Transaction<std::uint64_t> table {options};
    TeardownPSQL(table);
}

static void BM_DoubletsUnitedEachAllFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace SetupTeardown;
    std::filesystem::path path {"united.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage {FileMappedResizableDirectMemory{path.string()}};
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        storage.Each({any, any, any}, handler);
    }
    TeardownDoublets(storage);
}

static void BM_DoubletsUnitedEachAllRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace SetupTeardown;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(memory)};
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        storage.Each({any, any, any}, handler);
    }
    TeardownDoublets(storage);
}

static void BM_DoubletsSplitEachAllFile(benchmark::State& state) {
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
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        storage.Each({any, any, any}, handler);
    }
    TeardownDoublets(storage);
}

static void BM_DoubletsSplitEachAllRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace Platform::Collections;
    using namespace SetupTeardown;
    HeapResizableDirectMemory data {}, index {};
    SplitMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(data), std::move(index)};
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    SetupDoublets(storage);
    for (auto _: state) {
        storage.Each({any, any, any}, handler);
    }
    TeardownDoublets(storage);
}

BENCHMARK(BM_PSQLEachAllWithoutTransaction)->Name("BM_PSQL/Each/All/NonTransaction")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_PSQLEachAllWithTransaction)->Name("BM_PSQL/Each/All/Transaction")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsUnitedEachAllFile)->Name("BM_Doublets/United/Each/All/NonVolatile")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsUnitedEachAllRAM)->Name("BM_Doublets/United/Each/All/Volatile")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsSplitEachAllFile)->Name("BM_Doublets/Split/Each/All/NonVolatile")->Arg(1000)->MinWarmUpTime(10);
BENCHMARK(BM_DoubletsSplitEachAllRAM)->Name("BM_Doublets/Split/Each/All/Volatile")->Arg(1000)->MinWarmUpTime(10);
