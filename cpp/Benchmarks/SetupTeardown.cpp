namespace internal
{
    static void SetupPSQL(const benchmark::State& state) {
        Transaction<std::uint64_t> table (options);
        for (std::uint64_t i {}; i < BACKGROUND_LINKS; ++i) {
            table.CreatePoint();
        }
    }

    static void TeardownPSQL(const benchmark::State& state) {
        Transaction<std::uint64_t> table (options);
        table.DeleteAll();
    }
    
    static void SetupDoublets(const benchmark::State& state) {
        using namespace Platform::Memory;
        using namespace Platform::Data::Doublets;
        using namespace Memory::United::Generic;
        using namespace Platform::Collections;
        std::filesystem::path path {"db.links"};
        UnitedMemoryLinks<LinksOptions<std::uint64_t>> storage(FileMappedResizableDirectMemory(path.string()));
        for (std::uint64_t i {}; i < BACKGROUND_LINKS; ++i) {
            CreatePoint(storage);
        }
    }
    
    static void TeardownDoublets(const benchmark::State& state) {
        using namespace std::filesystem;
        remove(path{"db.links"});
    }
}
