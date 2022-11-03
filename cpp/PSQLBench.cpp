#include <benchmark/benchmark.h>
#include <Platform.Data.Doublets.h>

#include <Client.h>
#include <Transaction.h>

static void BM_PSQLCreateLinksWithoutTransaction(benchmark::State& state) {
    const std::string opts = "";
    for (auto _: state) {
        {
            Client<std::uint64_t> table(opts);
            for (std::uint64_t i {1}; i <= state.range(0); ++i) {
                std::vector substitution {i, i};
                table.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            Client<std::uint64_t> links(opts);
            for (std::uint64_t i {1}; i <= state.range(0); ++i) {
                std::vector restriction {i, i};
                links.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}

static void BM_PSQLCreateLinksWithTransaction(benchmark::State& state) {
    const std::string opts = "";
    for (auto _: state) {
        {
            Transaction<std::uint64_t> transaction(opts);
            for (std::uint64_t i {1}; i <= state.range(0); ++i) {
                std::vector substitution {i, i};
                transaction.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            Transaction<std::uint64_t> transaction(opts);
            for (std::uint64_t i {1}; i <= state.range(0); ++i) {
                std::vector restriction {i, i};
                transaction.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}

static void BM_DoubletsCreateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    std::filesystem::path path("db.links");
    for (auto _: state) {
        UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
        for (std::size_t i{}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        std::filesystem::remove(path);
        state.ResumeTiming();
    }
}

static void BM_DoubletsCreateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    for (auto _: state) {
        HeapResizableDirectMemory memory;
        UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
        for (std::size_t i{}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    }
}


BENCHMARK(BM_PSQLCreateLinksWithoutTransaction)->Arg(1000);
BENCHMARK(BM_PSQLCreateLinksWithTransaction)->Arg(1000);
BENCHMARK(BM_DoubletsCreateLinksFile)->Arg(1000);
BENCHMARK(BM_DoubletsCreateLinksRAM)->Arg(1000);
BENCHMARK_MAIN();
