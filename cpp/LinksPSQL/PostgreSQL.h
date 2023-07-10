#pragma once
#include <pqxx/pqxx>
#include <Platform.Data.h>

namespace PostgreSQL 
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;
    
    template <typename TExecutor>
    void Create(TExecutor& table, const LinkType& substitution)
    {
        table.executor().exec(
            "INSERT INTO Links(from_id, to_id) VALUES ("
            + table.executor().esc(std::to_string(substitution[1])) + ", "
            + table.executor().esc(std::to_string(substitution[2])) + ");"
        );
    }

    template <typename TExecutor>
    void CreatePoint(TExecutor& table)
    {
        table.executor().exec("INSERT INTO Links(from_id, to_id) VALUES (DEFAULT, DEFAULT);");
        table.executor().exec("UPDATE Links SET from_id = lastval(), to_id = lastval() WHERE id = lastval();");
    }
    
    template <typename TExecutor>
    void Update(TExecutor& table, const LinkType& restriction, const LinkType& substitution)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "UPDATE Links SET from_id = " + table.executor().esc(std::to_string(substitution[1]))
                    + ", to_id = " + table.executor().esc(std::to_string(substitution[2]))
                    + " WHERE id = " + table.executor().esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "UPDATE Links SET from_id = " + table.executor().esc(std::to_string(substitution[1]))
                    + ", to_id = " + table.executor().esc(std::to_string(substitution[2]))
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
            query = "DELETE FROM Links WHERE from_id = " + table.executor().esc(std::to_string(restriction[1]))
                    + " AND to_id = " + table.executor().esc(std::to_string(restriction[2])) + ";";
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
        }
        table.executor().exec(query);
    }

    template <typename TExecutor>
    void DeleteAll(TExecutor& table)
    {
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
    
    template <typename TExectuor>
    void Drop(TExectuor& table) {
        table.executor().exec("DROP TABLE Links;");
    }
};
