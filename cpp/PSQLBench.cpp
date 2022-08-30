#include <benchmark/benchmark.h>
#include <Platform.Data.Doublets.h>

#include <Client.h>
#include <Transaction.h>

static void BM_CreateThousandLinksWithoutTransaction(benchmark::State& state) {
    const std::string opts = "";
    for (auto _: state) {
        {
            Client<std::uint64_t> table(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector substitution{i, i};
                table.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            Client<std::uint64_t> links(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector restriction{i, i};
                links.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}

static void BM_CreateThousandLinksWithTransaction(benchmark::State& state) {
    const std::string opts = "";
    for (auto _: state) {
        {
            Transaction<std::uint64_t> transaction(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector substitution{i, i};
                transaction.Create(substitution);
            }
        }
        state.PauseTiming();
        {
            Transaction<std::uint64_t> transaction(opts);
            for (std::uint64_t i{1}; i <= state.range(0); ++i) {
                std::vector restriction{i, i};
                transaction.Delete(restriction);
            }
        }
        state.ResumeTiming();
    }
}

static void BM_CreateMillionPointsDoubletsFile(benchmark::State& state) {
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

static void BM_CreateMillionPointsDoubletsRAM(benchmark::State& state) {
    using namespace Platform::Memory;
    using namespace Platform::Data::Doublets;
    using namespace Memory::United::Generic;
    HeapResizableDirectMemory memory;
    for (auto _: state) {
        UnitedMemoryLinks<LinksOptions<std::uint64_t>, HeapResizableDirectMemory> storage(std::move(memory));
        for (std::size_t i{}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
        state.PauseTiming();
        std::vector<std::uint64_t> id_storage {};
        std::function handler = [&id_storage, &storage] (std::vector<std::uint64_t> ids) {
            id_storage.push_back(ids[0]);
            return storage.Constants.Continue;
        };
        storage.Each({storage.Constants.Any}, handler);
        for(std::uint64_t i{}, size { std::size(id_storage) - 1 }; i < size; ++i) {
            Update(storage, id_storage[i], 0, 0);
            Delete(storage, id_storage[i]);
        }
        state.ResumeTiming();
    }
}


BENCHMARK(BM_CreateThousandLinksWithoutTransaction)->Arg(1000);
BENCHMARK(BM_CreateThousandLinksWithTransaction)->Arg(1000);
BENCHMARK(BM_CreateMillionPointsDoubletsFile)->Arg(1000000);
BENCHMARK(BM_CreateMillionPointsDoubletsRAM)->Arg(1000000);
BENCHMARK_MAIN();
