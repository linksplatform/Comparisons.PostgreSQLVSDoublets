#include <array>
#include <vector>
#include <utility>

#include <pqxx/pqxx>
#include <Platform.Data.h>

template<typename TLink>
struct LinksPSQL
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;

    explicit LinksPSQL(std::string dbopts) : opts(std::move(dbopts))
    {
        line.retrieve(line.insert("CREATE TABLE IF NOT EXISTS Links(id bigint, from_id bigint, to_id bigint);"));
        GetIndex();
    }

    ~LinksPSQL()
    {
        this->Complete();
        connection.close();
    }

    TLink Create(const LinkType& substitution)
    {
        return line.insert("INSERT INTO Links VALUES (" + std::to_string(++index) + ", "
        + std::to_string(substitution[0]) + ", " + std::to_string(substitution[1]) + ");");
    }

    TLink Update(const LinkType& restrictions, const LinkType& substitution)
    {
        std::string query {};
        if (std::size(restrictions) == 1 || std::size(restrictions) == 3)
        {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE id = " + std::to_string(restrictions[0]) + ";";
        }
        else if (std::size(restrictions) == 2)
        {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE from_id = " + std::to_string(restrictions[0])
                    + " AND to_id = " + std::to_string(restrictions[1]) + ";";
        }
        return line.insert(query);
    }

    TLink Delete(const LinkType& restrictions)
    {
        std::string query {};
        if (std::size(restrictions) == 1 || std::size(restrictions) == 3)
        {
            query = "DELETE FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
        }
        else if (std::size(restrictions) == 2)
        {
            query = "DELETE FROM Links WHERE from_id = " + std::to_string(restrictions[0])
                    + " AND to_id = " + std::to_string(restrictions[1]) + ";";
        }
        return line.insert(query);
    }

    void Complete()
    {
        line.complete();
        transaction.commit();
    }

    TLink Count(const LinkType& restrictions)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query {};
        if (restrictions[0] == any && restrictions[1] == any && restrictions[2] == any)
            query = "SELECT COUNT(*) FROM Links;";
        else if (restrictions[0] != any && restrictions[1] == any && restrictions[2] == any)
            query = "SELECT COUNT(*) FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
        else if (restrictions[0] == any && restrictions[1] != any && restrictions[2] == any)
            query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + ";";
        else if (restrictions[0] == any && restrictions[1] == any && restrictions[2] != any)
            query = "SELECT COUNT(*) FROM Links WHERE to_id = " + std::to_string(restrictions[2]) + ";";
        else if (restrictions[0] == any && restrictions[1] != any && restrictions[2] != any)
            query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restrictions[1])
                    + "AND to_id = " + std::to_string(restrictions[2]) + ";";
        auto result = line.retrieve(line.insert(query));
        return result[0][0].as<TLink>();
    }

    std::vector<std::array<TLink, 3>> Each(const LinkType& restrictions)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query{};
        if (restrictions[0] == any && restrictions[1] == any && restrictions[2] == any)
            query = "SELECT * FROM Links;";
        else if (restrictions[0] != any && restrictions[1] == any && restrictions[2] == any)
            query = "SELECT * FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
        else if (restrictions[0] == any && restrictions[1] != any && restrictions[2] == any)
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + ";";
        else if (restrictions[0] == any && restrictions[1] == any && restrictions[2] != any)
            query = "SELECT * FROM Links WHERE to_id = " + std::to_string(restrictions[2]) + ";";
        else if (restrictions[0] == any && restrictions[1] != any && restrictions[2] != any)
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + "AND to_id = "
                    + std::to_string(restrictions[2]) + ";";
        auto result = line.retrieve(line.insert(query));
        std::vector<std::array<TLink, 3>> links {};
        for(int i{}; i<result.size(); ++i)
        {
            std::array<TLink, 3> link {};
            for(int j{}; j<3; j++) { link[j] = result[i][j].as<TLink>(); }
            links.push_back(link);
        }
        return links;
    }

    private: void GetIndex() { index = line.retrieve(line.insert("SELECT * FROM Links;")).size(); }

    private: std::string opts {};
    private: TLink index {};

    private: pqxx::connection connection {opts};
    private: pqxx::work transaction {connection};
    private: pqxx::pipeline line {transaction};
};
