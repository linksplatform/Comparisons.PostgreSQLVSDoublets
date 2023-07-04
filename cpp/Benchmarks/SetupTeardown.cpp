namespace SetupTeardown
{
    template <typename TStorage>
    void SetupDoublets(TStorage& storage) {
        using namespace Platform::Data::Doublets;
        for (std::uint64_t i {}; i < BACKGROUND_LINKS; ++i) {
            CreatePoint(storage);
        }
    }

    template <typename TStorage>
    void TeardownDoublets(TStorage& storage) {
        using namespace Platform::Data::Doublets;
        DeleteAll(storage);
    }

    template <typename TExecutor>
    void SetupPSQL(TExecutor& table) {
        using namespace PostgreSQL;
        for (std::uint64_t i {}; i < BACKGROUND_LINKS; ++i) {
            CreatePoint(table);
        }
    }

    template <typename TExecutor>
    static void TeardownPSQL(TExecutor& table) {
        using namespace PostgreSQL;
        DeleteAll(table);
    }
}
