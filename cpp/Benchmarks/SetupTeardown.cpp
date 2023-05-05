namespace internal
{
    static void SetupPSQL(const benchmark::State& state) {
        Transaction<std::uint64_t> table (options);
        for (std::uint64_t i = 1; i <= state.range(0); ++i) {
            table.CreatePoint();
        }
    }

    static void TeardownPSQL(const benchmark::State& state) {
        Transaction<std::size_t> table (options);
        table.DeleteAll();
    }

    static void SetupDoubletsFile(const benchmark::State& state) {
        using namespace Platform::Memory;
        using namespace Platform::Data::Doublets;
        using namespace Memory::United::Generic;
        using namespace Platform::Collections;
        std::filesystem::path path {"db.links"};
        UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
        for (std::uint64_t i {}; i < state.range(0); ++i) {
            CreatePoint(storage);
        }
    }

    static void TeardownDoubletsFile(const benchmark::State& state) {
        using namespace Platform::Memory;
        using namespace Platform::Data::Doublets;
        using namespace Memory::United::Generic;
        using namespace Platform::Collections;
        std::filesystem::path path {"db.links"};
        UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
        DeleteAll(storage);
    }
}
