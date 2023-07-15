#pragma once

namespace PostgreSQL
{

    template<typename TLinksOptions>
    struct Transaction: public PSQLBase<TLinksOptions, pqxx::transaction<>>
    {
        using base = PSQLBase<TLinksOptions, pqxx::transaction<>>;
        using LinksOptions = TLinksOptions;
        using LinkType = typename TLinksOptions::LinkType;
        using LinkAddressType = typename TLinksOptions::LinkAddressType;
        using base::Constants;
        
        explicit Transaction(const std::string& opts): base(opts)
        {
        }
    };
}
