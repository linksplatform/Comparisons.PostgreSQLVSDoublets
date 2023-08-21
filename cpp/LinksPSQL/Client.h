#pragma once

namespace PostgreSQL
{
    
    template<typename TLinksOptions>
    struct Client: public PSQLBase<TLinksOptions, pqxx::nontransaction>
    {
        using base = PSQLBase<TLinksOptions, pqxx::nontransaction>;
        using LinksOptions = TLinksOptions;
        using LinkType = typename TLinksOptions::LinkType;
        using LinkAddressType = typename TLinksOptions::LinkAddressType;
        using base::base;
    };
}
