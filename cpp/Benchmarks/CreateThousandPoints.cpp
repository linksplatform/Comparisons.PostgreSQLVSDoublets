#include <benchmark/benchmark.h>
#include "Platform.Data.Doublets.h"

#include "Client.h"
#include "Transaction.h"

const std::string opts = "";


static void BM_PSQLCreateLinksWithoutTransaction(benchmark::State& state) {
    Client<std::uint64_t> table(opts);
    for (auto _: state) {
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.Create({i, i});
        }
        state.PauseTiming();
        table.DeleteAll();
        state.ResumeTiming();
    }
}

static void BM_PSQLCreateLinksWithTransaction(benchmark::State& state) {
    std::chrono::time_point<std::chrono::system_clock, std::chrono::nanoseconds> start;
    for (auto _: state) {
        {
            Transaction<std::uint64_t> transaction(opts);
            start = std::chrono::high_resolution_clock::now();
            for (std::uint64_t i = 1; i <= state.range(0); ++i) {
                transaction.Create({i, i});
            }
        }
        auto stop = std::chrono::high_resolution_clock::now();
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<std::double_t>>(stop-start).count();
        {
            Transaction<std::uint64_t> transaction(opts);
            transaction.DeleteAll();
        }
        state.SetIterationTime(elapsed_time);
    }
}

static void BM_DoubletsCreateLinksFile(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    using namespace Platform::Collections;
    std::filesystem::path path("db.links");
    UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
    for (auto _: state) {
        auto start = std::chrono::high_resolution_clock::now();
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        auto stop = std::chrono::high_resolution_clock::now();
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<std::double_t>>(stop-start).count();
        DeleteAll(storage);
        state.SetIterationTime(elapsed_time);
    }
}

static void BM_DoubletsCreateLinksRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    HeapResizableDirectMemory memory {};
    UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
    for (auto _: state) {
        auto start = std::chrono::high_resolution_clock::now();
        for (std::size_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        auto stop = std::chrono::high_resolution_clock::now();
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<std::double_t>>(stop-start).count();
        DeleteAll(storage);
        state.SetIterationTime(elapsed_time);
    }
}


BENCHMARK(BM_PSQLCreateLinksWithoutTransaction)->Arg(1000);
BENCHMARK(BM_PSQLCreateLinksWithTransaction)->Arg(1000)->UseManualTime();
BENCHMARK(BM_DoubletsCreateLinksRAM)->Arg(1000)->UseManualTime();
BENCHMARK(BM_DoubletsCreateLinksFile)->Arg(1000)->UseManualTime();
