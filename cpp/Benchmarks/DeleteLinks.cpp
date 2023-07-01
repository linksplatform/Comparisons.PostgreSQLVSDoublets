static void BM_PSQLDeleteLinksWithoutTransaction(benchmark::State& state) {
    Client<std::uint64_t> table(options);
    auto setup = [&table, &state] {
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            table.CreatePoint();
        }
    };
    for (auto _: state) {
        state.PauseTiming();
        setup();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i != BACKGROUND_LINKS; --i) {
            table.Delete({i});
        }
    }
}

static void BM_PSQLDeleteLinksWithTransaction(benchmark::State& state) {
    auto setup = [&options, &state] {
        Transaction<std::uint64_t> transaction(options);
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            transaction.CreatePoint();
        }
    };
    for (auto _: state) {
        state.PauseTiming();
        setup();
        Transaction<std::uint64_t> transaction(options);
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS+state.range(0); i > BACKGROUND_LINKS; --i) {
            transaction.Delete({i});
        }
    }
}

static void BM_DoubletsDeleteLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    std::filesystem::path path("db.links");
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    auto setup = [&storage, &state] {
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    for (auto _: state) {
        state.PauseTiming();
        setup();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i != BACKGROUND_LINKS; --i) {
            storage.Delete({i}, handler);
        }
    }
}

static void BM_DoubletsDeleteLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
    auto any = storage.Constants.Any;
    auto handler = [&storage] (std::vector<std::uint64_t> before, std::vector<std::uint64_t> after) {
        return storage.Constants.Continue;
    };
    auto setup = [&storage, &state] {
        for (std::size_t i {}; i != state.range(0); ++i) {
            CreatePoint(storage);
        }
    };
    for (std::size_t i {}; i < BACKGROUND_LINKS; ++i) {
        CreatePoint(storage);
    }
    for (auto _: state) {
        state.PauseTiming();
        setup();
        state.ResumeTiming();
        for (std::uint64_t i = BACKGROUND_LINKS + state.range(0); i != BACKGROUND_LINKS; --i) {
            storage.Delete({i}, handler);
        }
    }
}

BENCHMARK(BM_PSQLDeleteLinksWithoutTransaction)->Name("BM_PSQL/DeleteLinks/NoTransaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_PSQLDeleteLinksWithTransaction)->Name("BM_PSQL/DeleteLinks/Transaction")->Arg(1000)->Setup(internal::SetupPSQL)->Teardown(internal::TeardownPSQL);
BENCHMARK(BM_DoubletsDeleteLinksRAM)->Name("BM_Doublets/DeleteLinks/Volatile")->Arg(1000);
BENCHMARK(BM_DoubletsDeleteLinksFile)->Name("BM_Doublets/DeleteLinks/NonVolatile")->Arg(1000)->Setup(internal::SetupDoublets)->Teardown(internal::TeardownDoublets);
