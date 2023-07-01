static void BM_PSQLEachAllWithoutTransaction(benchmark::State& state) {
    using namespace Platform::Data;
    Client<std::uint64_t> table {options};
    auto any = LinksConstants<std::uint64_t>().Any;
    for (auto _: state) {
        table.Each({any, any, any});
    }
}

static void BM_PSQLEachAllWithTransaction(benchmark::State& state) {
    using namespace Platform::Data;
    auto any = LinksConstants<std::uint64_t>().Any;
    for (auto _: state) {
        state.PauseTiming();
        Transaction<std::uint64_t> table {options};
        state.ResumeTiming();
        table.Each({any, any, any});
    }
}

static void BM_DoubletsEachAllFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    std::filesystem::path path {"db.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage (FileMappedResizableDirectMemory(path.string()));
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> vec) {
        return storage.Constants.Continue;
    };
    for (auto _: state) {
        storage.Each({any, any, any}, handler);
    }
}

static void BM_DoubletsEachAllRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
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
        storage.Each({any, any, any}, handler);
    }
}

BENCHMARK(BM_PSQLEachAllWithoutTransaction)->Name("BM_PSQL/Each/All/NonTransaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLEachAllWithTransaction)->Name("BM_PSQL/Each/All/Transaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsEachAllRAM)->Name("BM_Doublets/Each/All/Volatile")->Arg(1000);
BENCHMARK(BM_DoubletsEachAllFile)->Name("BM_Doublets/Each/All/NonVolatile")->Arg(1000)->Setup(internal::SetupDoublets)->Teardown(internal::TeardownDoublets);
