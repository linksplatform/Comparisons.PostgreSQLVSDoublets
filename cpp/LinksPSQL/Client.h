#pragma once
#include <array>
#include <vector>

#include <pqxx/pqxx>

template<typename TLink>
struct Client
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;

    explicit Client(std::string opts) : connection(opts)
    {
        client.exec("CREATE TABLE IF NOT EXISTS Links (id bigint PRIMARY KEY, from_id bigint, to_id bigint);");
        client.exec("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);");
        client.exec("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);");
        index = client.exec("SELECT * FROM LINKS").size();
    }

    ~Client()
    {
        connection.close();
    };
    
    pqxx::nontransaction& executor() {
        return client;
    }
    
    TLink index {};

    private: pqxx::connection connection;
    private: pqxx::nontransaction client {connection};
};
