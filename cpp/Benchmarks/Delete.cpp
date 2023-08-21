static void BM_PSQLDeleteLinksWithoutTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    Client<LinksOptions<std::uint64_t>> table {options};
    Setup(table);
    auto setup = [&table, &state] {
        for (std::uint64_t i = BACKGROUND_LINKS + 1; i <= BACKGROUND_LINKS + state.range(0); ++i) {
            CreatePoint(table);
        }
    };
    auto background {BACKGROUND_LINKS - state.range(0)};
    for (auto _: state) {
        state.PauseTiming();
        setup();
        background += state.range(0);
        state.ResumeTiming();
        for (std::uint64_t i = background + state.range(0); i > background; --i) {
            Delete(table, {i, i, i});
        }
    }
    Teardown(table);
}

static void BM_PSQLDeleteLinksWithTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    {
        Transaction<LinksOptions<std::uint64_t>> transaction {options};
        Setup(transaction);
    }
    auto background {BACKGROUND_LINKS - state.range(0)};
    auto fill = [options = options, &state] {
        Transaction<LinksOptions<std::uint64_t>> transaction {options};
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            CreatePoint(transaction);
        }
    };
    for (auto _: state) {
        state.PauseTiming();
        fill();
        background += state.range(0);
        Transaction<LinksOptions<std::uint64_t>> transaction {options};
        state.ResumeTiming();
        for (std::uint64_t i = background + state.range(0); i > background; --i) {
            Delete(transaction, {i, i, i});
        }
    }
    Transaction<LinksOptions<std::uint64_t>> transaction {options};
    Teardown(transaction);
}

static void BM_DoubletsUnitedDeleteLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    using namespace SetupTeardown;
    std::filesystem::path path {"united.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage {FileMappedResizableDirectMemory{path.string()}};
    auto fill = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    Setup(storage);
    for (auto _: state) {
        state.PauseTiming();
        fill();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            Delete(storage, {i, i, i});
        }
    }
    Teardown(storage);
}

static void BM_DoubletsUnitedDeleteLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(memory)};
    auto fill = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    Setup(storage);
    for (auto _: state) {
        state.PauseTiming();
        fill();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            Delete(storage, {i, i, i});
        }
    }
    Teardown(storage);
}

static void BM_DoubletsSplitDeleteLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    std::filesystem::path split_index {"split_index.links"}, split_data {"split_data.links"};
    SplitMemoryLinks<LinksOptions<std::uint64_t>> storage {
        FileMappedResizableDirectMemory{split_data.string()},
        FileMappedResizableDirectMemory{split_index.string()}
    };
    auto fill = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    Setup(storage);
    for (auto _: state) {
        state.PauseTiming();
        fill();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            Delete(storage, {i, i, i});
        }
    }
    Teardown(storage);
}

static void BM_DoubletsSplitDeleteLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory index {}, data {};
    SplitMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(data), std::move(index)};
    auto fill = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    Setup(storage);
    for (auto _: state) {
        state.PauseTiming();
        fill();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i > BACKGROUND_LINKS; --i) {
            Delete(storage, {i, i, i});
        }
    }
    Teardown(storage);
}

BENCHMARK(BM_PSQLDeleteLinksWithoutTransaction)->Name("BM_PSQL/Delete/NonTransaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_PSQLDeleteLinksWithTransaction)->Name("BM_PSQL/Delete/Transaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedDeleteLinksFile)->Name("BM_Doublets/United/Delete/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedDeleteLinksRAM)->Name("BM_Doublets/United/Delete/Volatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitDeleteLinksFile)->Name("BM_Doublets/Split/Delete/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitDeleteLinksRAM)->Name("BM_Doublets/Split/Delete/Volatile")->Arg(1000)->MinWarmUpTime(20);
