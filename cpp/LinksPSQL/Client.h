#pragma once
#include <array>
#include <vector>

#include <pqxx/pqxx>

template<typename TLink>
struct Client
{
    explicit Client(std::string opts) : connection(opts)
    {
        client.exec("CREATE TABLE IF NOT EXISTS Links (id bigint PRIMARY KEY, from_id bigint, to_id bigint);");
        client.exec("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);");
        client.exec("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);");
        auto result = client.exec(
            "SELECT * FROM links\n"
            "ORDER BY id DESC\n"
            "LIMIT 1"
        );
        if (!result.empty()) {
            index = result[0][0].as<TLink>();
        }
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
