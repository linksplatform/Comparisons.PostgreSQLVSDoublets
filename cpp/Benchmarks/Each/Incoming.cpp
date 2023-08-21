static void BM_PSQLEachIncomingWithoutTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    Client<LinksOptions<std::uint64_t>> table {options};
    auto any {table.Constants.Any}, $continue {table.Constants.Continue};
    auto handler = [$continue] (std::vector<std::uint64_t> vec) {
        return $continue;
    };
    Setup(table);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            table.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            table.Each({any, any, i}, handler);
        }
    }
    Teardown(table);
}

static void BM_PSQLEachIncomingWithTransaction(benchmark::State& state) {
    using namespace SetupTeardown;
    using namespace Platform::Data::Doublets;
    using namespace PostgreSQL;
    std::uint64_t any {}, $continue {};
    {
        Transaction<LinksOptions<std::uint64_t>> table {options};
        Setup(table);
        any = table.Constants.Any;
        $continue = table.Constants.Continue;
    }
    auto handler = [$continue] (std::vector<std::uint64_t> vec) {
        return $continue;
    };
    for (auto _: state) {
        state.PauseTiming();
        Transaction<LinksOptions<std::uint64_t>> table {options};
        state.ResumeTiming();
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            table.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            table.Each({any, any, i}, handler);
        }
    }
    Transaction<LinksOptions<std::uint64_t>> table {options};
    Teardown(table);
}

static void BM_DoubletsUnitedEachIncomingFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    std::filesystem::path path {"united.links"};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage {FileMappedResizableDirectMemory{path.string()}};
    auto any {storage.Constants.Any}, $continue {storage.Constants.Continue};
    auto handler = [$continue] (std::vector<std::uint64_t> vec) {
        return $continue;
    };
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Each({any, any, i}, handler);
        }
    }
    Teardown(storage);
}

static void BM_DoubletsUnitedEachIncomingRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(memory)};
    auto any {storage.Constants.Any}, $continue {storage.Constants.Continue};
    auto handler = [$continue] (std::vector<std::uint64_t> vec) {
        return $continue;
    };
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Each({any, any, i}, handler);
        }
    }
    Teardown(storage);
}

static void BM_DoubletsSplitEachIncomingFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    std::filesystem::path split_data {"split_data.links"}, split_index {"split_index.links"};
    SplitMemoryLinks<LinksOptions<std::uint64_t>> storage {
        FileMappedResizableDirectMemory{split_data.string()},
        FileMappedResizableDirectMemory{split_index.string()}
    };
    auto any {storage.Constants.Any}, $continue {storage.Constants.Continue};
    auto handler = [$continue] (std::vector<std::uint64_t> vec) {
        return $continue;
    };
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Each({any, any, i}, handler);
        }
    }
    Teardown(storage);
}

static void BM_DoubletsSplitEachIncomingRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::Split::Generic;
    using namespace SetupTeardown;
    HeapResizableDirectMemory data {}, index {};
    SplitMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage {std::move(data), std::move(index)};
    auto any {storage.Constants.Any}, $continue {storage.Constants.Continue};
    auto handler = [$continue] (std::vector<std::uint64_t> vec) {
        return $continue;
    };
    Setup(storage);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS/2 - state.range(0)/2 + 1; i <= BACKGROUND_LINKS/2 + state.range(0)/2; ++i) {
            storage.Each({any, any, i}, handler);
        }
        for (std::uint64_t i = BACKGROUND_LINKS - state.range(0) + 1; i <= BACKGROUND_LINKS; ++i) {
            storage.Each({any, any, i}, handler);
        }
    }
    Teardown(storage);
}

BENCHMARK(BM_PSQLEachIncomingWithoutTransaction)->Name("BM_PSQL/Each/Incoming/NonTransaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_PSQLEachIncomingWithTransaction)->Name("BM_PSQL/Each/Incoming/Transaction")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedEachIncomingFile)->Name("BM_Doublets/United/Each/Incoming/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsUnitedEachIncomingRAM)->Name("BM_Doublets/United/Each/Incoming/Volatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitEachIncomingFile)->Name("BM_Doublets/Split/Each/Incoming/NonVolatile")->Arg(1000)->MinWarmUpTime(20);
BENCHMARK(BM_DoubletsSplitEachIncomingRAM)->Name("BM_Doublets/Split/Each/Incoming/Volatile")->Arg(1000)->MinWarmUpTime(20);
