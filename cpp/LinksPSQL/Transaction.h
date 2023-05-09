#pragma once
#include <array>
#include <vector>
#include <format>
#include <pqxx/pqxx>
#include <Platform.Data.h>

template<typename TLink>
struct Transaction
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;

    explicit Transaction(std::string opts) : connection(opts)
    {
        transaction.exec("CREATE TABLE IF NOT EXISTS Links (id bigint PRIMARY KEY, from_id bigint, to_id bigint);");
        transaction.exec("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);");
        transaction.exec("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);");
    }

    ~Transaction()
    {
        transaction.commit();
        connection.close();
    };

    void Create(const LinkType& substitution)
    {
        transaction.exec("INSERT INTO Links VALUES ("
                        + transaction.esc(std::to_string(++index)) + ", "
                        + transaction.esc(std::to_string(substitution[0])) + ", "
                        + transaction.esc(std::to_string(substitution[1]))+ ");");
    }

    void CreatePoint()
    {
        transaction.exec("INSERT INTO Links VALUES (" + transaction.esc(std::to_string(++index)) + ", "
                        + transaction.esc(std::to_string(index)) + ", "
                        + transaction.esc(std::to_string(index)) + ");");
    }

    void Update(const LinkType& restriction, const LinkType& substitution)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "UPDATE Links SET from_id = " + transaction.esc(std::to_string(substitution[0]))
                    + ", to_id = " + transaction.esc(std::to_string(substitution[1]))
                    + " WHERE id = " + transaction.esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "UPDATE Links SET from_id = " + transaction.esc(std::to_string(substitution[0]))
                    + ", to_id = " + transaction.esc(std::to_string(substitution[1]))
                    + " WHERE from_id = " + transaction.esc(std::to_string(restriction[1]))
                    + " AND to_id = " + transaction.esc(std::to_string(restriction[2])) + ";";
        }
        transaction.exec(query);
    }

    void Delete(const LinkType& restriction)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "DELETE FROM Links WHERE id = " + transaction.esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "DELETE FROM Links WHERE from_id = " + transaction.esc(std::to_string(restriction[0]))
                    + " AND to_id = " + transaction.esc(std::to_string(restriction[1])) + ";";
        }
        transaction.exec(query);
    }
    
    void DeleteAll()
    {
        transaction.exec("DELETE FROM LINKS");
    }
    
    TLink Count(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT COUNT(*) FROM Links WHERE ";
        std::string id =  restriction[0] == any ? "" : std::format("id = {} AND ", transaction.esc(std::to_string(restriction[0])));
        std::string source = restriction[1] == any ? "" : std::format("from_id = {} AND ", transaction.esc(std::to_string(restriction[1])));
        std::string target = restriction[2] == any ? "true;" : std::format("to_id = {};", transaction.esc(std::to_string(restriction[2])));
        query.append(id + source + target);
        pqxx::result result = transaction.exec(query);
        return result[0][0].as<TLink>();
    }

    std::vector<std::array<TLink, 3>> Each(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT * FROM Links WHERE ";
        std::string id =  restriction[0] == any ? "" : std::format("id = {} AND ", transaction.esc(std::to_string(restriction[0])));
        std::string source = restriction[1] == any ? "" : std::format("from_id = {} AND ", transaction.esc(std::to_string(restriction[1])));
        std::string target = restriction[2] == any ? "true;" : std::format("to_id = {};", transaction.esc(std::to_string(restriction[2])));
        query.append(id + source + target);
        pqxx::result result = transaction.exec(query);
        std::vector<std::array<TLink, 3>> links{};
        for (std::size_t i {}; i < result.size(); ++i) {
            links.push_back({result[i][0].as<TLink>(), result[i][1].as<TLink>(), result[i][2].as<TLink>()});
        }
        return links;
    }
    
    private: TLink index {};

    private: pqxx::connection connection;
    private: pqxx::transaction<> transaction {connection};
};
