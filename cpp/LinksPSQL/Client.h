#pragma once
#include <array>
#include <vector>

#include <pqxx/pqxx>
#include <Platform.Data.h>

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

    void Create(const LinkType& substitution)
    {
        ++index;
        client.exec("INSERT INTO Links VALUES (" 
                   + client.esc(std::to_string(index)) + ", "
                   + client.esc(std::to_string(substitution[0])) + ", "
                   + client.esc(std::to_string(substitution[1]))+ ");");
    }

    void CreatePoint()
    {
        ++index;
        client.exec("INSERT INTO Links VALUES (" + client.esc(std::to_string(index)) + ", "
                   + client.esc(std::to_string(index)) + ", "
                   + client.esc(std::to_string(index)) + ");");
    }
    
    void Update(const LinkType& restriction, const LinkType& substitution)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "UPDATE Links SET from_id = " + client.esc(std::to_string(substitution[0]))
                    + ", to_id = " + client.esc(std::to_string(substitution[1])) 
                    + " WHERE id = " + client.esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "UPDATE Links SET from_id = " + client.esc(std::to_string(substitution[0]))
                    + ", to_id = " + client.esc(std::to_string(substitution[1]))
                    + " WHERE from_id = " + client.esc(std::to_string(restriction[1]))
                    + " AND to_id = " + client.esc(std::to_string(restriction[2])) + ";";
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
        }
        client.exec(query);
    }

    void Delete(const LinkType& restriction)
    {
        std::string query {};
        if (std::size(restriction) == 1) {
            query = "DELETE FROM Links WHERE id = " + client.esc(std::to_string(restriction[0])) + ";";
        } else if (std::size(restriction) == 3) {
            query = "DELETE FROM Links WHERE from_id = " + client.esc(std::to_string(restriction[0])) 
                    + " AND to_id = " + client.esc(std::to_string(restriction[1])) + ";";
        } else {
            throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
        }
        client.exec(query);
        index = client.exec("SELECT * FROM LINKS").size();
    }

    void DeleteAll()
    {
        client.exec("DELETE FROM LINKS;");
        index = 0;
    }

    TLink Count(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT COUNT(*) FROM Links WHERE";
        std::string id =  restriction[0] == any ? "" : " id = " + client.esc(std::to_string(restriction[0])) + " AND";
        std::string source = restriction[1] == any ? "" : " from_id = " + client.esc(std::to_string(restriction[1])) + " AND";
        std::string target = restriction[2] == any ? " true;" : " to_id = " + client.esc(std::to_string(restriction[2])) + ';';
        query.append(id + source + target);
        pqxx::result result = client.exec(query);
        return result[0][0].as<TLink>();
    }

    std::vector<std::array<TLink, 3>> Each(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query = "SELECT * FROM Links WHERE";
        std::string id =  restriction[0] == any ? "" : " id = " + client.esc(std::to_string(restriction[0])) + " AND";
        std::string source = restriction[1] == any ? "" : " from_id = " + client.esc(std::to_string(restriction[1])) + " AND";
        std::string target = restriction[2] == any ? " true;" : " to_id = " + client.esc(std::to_string(restriction[2])) + ";";
        query.append(id + source + target);
        pqxx::result result = client.exec(query);
        std::vector<std::array<TLink, 3>> links{};
        for (std::size_t i {}; i < result.size(); ++i) {
            links.push_back({result[i][0].as<TLink>(), result[i][1].as<TLink>(), result[i][2].as<TLink>()});
        }
        return links;
    }
    
    private: TLink index {};

    private: pqxx::connection connection;
    private: pqxx::nontransaction client {connection};
};
