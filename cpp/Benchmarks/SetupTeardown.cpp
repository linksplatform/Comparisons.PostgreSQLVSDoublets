namespace SetupTeardown
{
    
    template <typename TStorage>
    void Setup(TStorage& storage) {
        using namespace Platform::Data::Doublets;
        for (std::uint64_t i {}; i < BACKGROUND_LINKS; ++i) {
            CreatePoint(storage);
        }
    }

    template <template <typename, typename...> class TStorage, typename TLinksOptions, typename ...TOther>
    void Teardown(TStorage<TLinksOptions, TOther...>& storage) {
        using namespace Platform::Data::Doublets;
        using namespace PostgreSQL;
        using namespace std::filesystem;
        DeleteAll(storage);
        remove(path{"united.links"});
        remove(path{"split_data.links"});
        remove(path{"split_index.links"});
        if constexpr (
            std::derived_from<TStorage<TLinksOptions, TOther...>, PSQLBase<TLinksOptions, pqxx::nontransaction>> ||
            std::derived_from<TStorage<TLinksOptions, TOther...>, PSQLBase<TLinksOptions, pqxx::transaction<>>>
        ) {
            storage.Drop();
        }
    }
}
