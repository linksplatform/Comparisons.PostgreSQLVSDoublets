#pragma once
#include <pqxx/pqxx>
#include <Platform.Data.h>

namespace PostgreSQL 
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;
    
    template <typename TExecutor>
    void Create(TExecutor& table, const LinkType& substitution)
    {
        ++table.index;
        table.executor().exec(
            "INSERT INTO Links VALUES ("
            + table.executor().esc(std::to_string(table.index)) + ", "
            + table.executor().esc(std::to_string(substitution[0])) + ", "
            + table.executor().esc(std::to_string(substitution[1])) + ");"
        );
    }

    template <typename TExecutor>
    void CreatePoint(TExecutor& table)
    {
        ++table.index;
        table.executor().exec(
            "INSERT INTO Links VALUES (" 
            + table.executor().esc(std::to_string(table.index)) + ", "
            + table.executor().esc(std::to_string(table.index)) + ", "
            + table.executor().esc(std::to_string(table.index)) + ");"
        );
    }

    template <typename TExecutor>
    void Update(TExecutor& table, const LinkType& restriction, const LinkType& substitution)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "UPDATE Links SET from_id = " + table.executor().esc(std::to_string(substitution[0]))
                    + ", to_id = " + table.executor().esc(std::to_string(substitution[1]))
                    + " WHERE id = " + table.executor().esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "UPDATE Links SET from_id = " + table.executor().esc(std::to_string(substitution[0]))
                    + ", to_id = " + table.executor().esc(std::to_string(substitution[1]))
                    + " WHERE from_id = " + table.executor().esc(std::to_string(restriction[1]))
                    + " AND to_id = " + table.executor().esc(std::to_string(restriction[2])) + ";";
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
        }
        table.executor().exec(query);
    }

    template <template <typename> class TExecutor, typename TLink>
    void Delete(TExecutor<TLink>& table, const LinkType& restriction)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "DELETE FROM Links WHERE id = " + table.executor().esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "DELETE FROM Links WHERE from_id = " + table.executor().esc(std::to_string(restriction[0]))
                    + " AND to_id = " + table.executor().esc(std::to_string(restriction[1])) + ";";
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
        }
        table.executor().exec(query);
        pqxx::result result = table.executor().exec(
            "SELECT * FROM links\n"
            "ORDER BY id DESC\n"
            "LIMIT 1"
        );
        if (!result.empty()) {
            table.index = result[0][0].as<TLink>();
        }
    }

    template <typename TExecutor>
    void DeleteAll(TExecutor& table)
    {
        table.index = 0;
        table.executor().exec("DELETE FROM LINKS;");
    }

    template <template <typename> class TExecutor, typename TLink>
    TLink Count(TExecutor<TLink>& table, const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT COUNT(*) FROM Links WHERE ";
        std::string id = restriction[0] == any ? "" : "id = " + table.executor().esc(std::to_string(restriction[0])) + " AND ";
        std::string source = restriction[1] == any ? "" : "from_id = " + table.executor().esc(std::to_string(restriction[1])) + " AND ";
        std::string target = restriction[2] == any ? "true;" : "to_id = " + table.executor().esc(std::to_string(restriction[2])) + ";";
        query.append(id + source + target);
        return table.executor().exec(query)[0][0].template as<TLink>();
    }

    template <template <typename> class TExecutor, typename TLink>
    std::vector<std::array<TLink, 3>> Each(TExecutor<TLink>& table, const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT * FROM Links WHERE ";
        std::string id = restriction[0] == any ? "" : "id = " + table.executor().esc(std::to_string(restriction[0])) + " AND ";
        std::string source = restriction[1] == any ? "" : "from_id = " + table.executor().esc(std::to_string(restriction[1])) + " AND ";
        std::string target = restriction[2] == any ? "true;" : "to_id = " + table.executor().esc(std::to_string(restriction[2])) + ";";
        query.append(id + source + target);
        pqxx::result result = table.executor().exec(query);
        std::vector<std::array<TLink, 3>> links{};
        links.reserve(result.size());
        for (const auto& row: result) {
            links.push_back({row[0].as<TLink>(), row[1].as<TLink>(), row[2].as<TLink>()});
        }
        return links;
    }
};