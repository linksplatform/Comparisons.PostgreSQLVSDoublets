#pragma once
#include <array>
#include <vector>

#include <pqxx/pqxx>

template<typename TLink>
struct Transaction
{
    explicit Transaction(std::string opts) : connection(opts)
    {
        transaction.exec("CREATE TABLE IF NOT EXISTS Links (id bigint PRIMARY KEY, from_id bigint, to_id bigint);");
        transaction.exec("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);");
        transaction.exec("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);");
        auto result = transaction.exec(
            "SELECT * FROM links\n"
            "ORDER BY id DESC\n"
            "LIMIT 1"
        );
        if (!result.empty()) {
            index = result[0][0].as<TLink>();
        }
    }

    ~Transaction()
    {
        transaction.commit();
        connection.close();
    };
    
    pqxx::transaction<>& executor() {
        return transaction;
    }
    
    TLink index {};

    private: pqxx::connection connection;
    private: pqxx::transaction<> transaction {connection};
};
