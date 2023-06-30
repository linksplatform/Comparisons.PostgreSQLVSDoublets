#pragma once
#include <array>
#include <vector>

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
        index = transaction.exec("SELECT * FROM Links;").size();
    }

    ~Transaction()
    {
        transaction.commit();
        connection.close();
    };

    void Create(const LinkType& substitution)
    {
        ++index;
        transaction.exec("INSERT INTO Links VALUES ("
                        + transaction.esc(std::to_string(index)) + ", "
                        + transaction.esc(std::to_string(substitution[0])) + ", "
                        + transaction.esc(std::to_string(substitution[1]))+ ");");
    }

    void CreatePoint()
    {
        ++index;
        transaction.exec("INSERT INTO Links VALUES (" + transaction.esc(std::to_string(index)) + ", "
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
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
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
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
        }
        transaction.exec(query);
    }
    
    void DeleteAll()
    {
        transaction.exec("DELETE FROM LINKS;");
        index = 0;
    }
    
    TLink Count(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT COUNT(*) FROM Links WHERE ";
        std::string id = restriction[0] == any ? "" : "id = " + transaction.esc(std::to_string(restriction[0])) + " AND ";
        std::string source = restriction[1] == any ? "" : "from_id = " + transaction.esc(std::to_string(restriction[1])) + " AND ";
        std::string target = restriction[2] == any ? "true;" : "to_id = " + transaction.esc(std::to_string(restriction[2])) + ";";
        query.append(id + source + target);
        pqxx::result result = transaction.exec(query);
        return result[0][0].as<TLink>();
    }

    std::vector<std::array<TLink, 3>> Each(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT * FROM Links WHERE ";
        std::string id = restriction[0] == any ? "" : "id = " + transaction.esc(std::to_string(restriction[0])) + " AND ";
        std::string source = restriction[1] == any ? "" : "from_id = " + transaction.esc(std::to_string(restriction[1])) + " AND ";
        std::string target = restriction[2] == any ? "true;" : "to_id = " + transaction.esc(std::to_string(restriction[2])) + ";";
        query.append(id + source + target);
        pqxx::result result = transaction.exec(query);
        std::vector<std::array<TLink, 3>> links{};
        links.reserve(result.size());
        for (const auto& row: result) {
            links.push_back({row[0].as<TLink>(), row[1].as<TLink>(), row[2].as<TLink>()});
        }
        return links;
    }
    
    private: TLink index {};

    private: pqxx::connection connection;
    private: pqxx::transaction<> transaction {connection};
};
