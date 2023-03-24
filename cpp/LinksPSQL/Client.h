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
        index = client.exec("SELECT * FROM Links;").size();
    }

    ~Client() 
    {
        connection.close();
    };

    void Create(const LinkType& substitution)
    {
        client.exec("INSERT INTO Links VALUES (" + std::to_string(++index) + ", "
                    + std::to_string(substitution[0]) + ", " + std::to_string(substitution[1]) + ");");
    }

    void Update(const LinkType& restriction, const LinkType& substitution)
    {
        std::string_view query{};
        if (std::size(restriction) == 1 || std::size(restriction) == 3) {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE id = " + std::to_string(restriction[0]) + ";";
        } else if (std::size(restriction) == 2) {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE from_id = " + std::to_string(restriction[0])
                    + " AND to_id = " + std::to_string(restriction[1]) + ";";
        }
        client.exec(query);
    }

    void Delete(const LinkType& restriction)
    {
        std::string_view query {};
        if (std::size(restriction) == 1 || std::size(restriction) == 3) {
            query = "DELETE FROM Links WHERE id = " + std::to_string(restriction[0]) + ";";
        } else if (std::size(restriction) == 2) {
            query = "DELETE FROM Links WHERE from_id = " + std::to_string(restriction[0])
                    + " AND to_id = " + std::to_string(restriction[1]) + ";";
        }
        client.exec(query);
    }

    void DeleteAll()
    {
        client.exec("DELETE FROM LINKS");
    }

    TLink Count(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string_view query {};
        if (restriction[0] == any && restriction[1] == any && restriction[2] == any)
            query = "SELECT COUNT(*) FROM Links;";
        else if (restriction[0] != any && restriction[1] == any && restriction[2] == any)
            query = "SELECT COUNT(*) FROM Links WHERE id = " + std::to_string(restriction[0]) + ";";
        else if (restriction[0] == any && restriction[1] != any && restriction[2] == any)
            query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restriction[1]) + ";";
        else if (restriction[0] == any && restriction[1] == any && restriction[2] != any)
            query = "SELECT COUNT(*) FROM Links WHERE to_id = " + std::to_string(restriction[2]) + ";";
        else if (restriction[0] == any && restriction[1] != any && restriction[2] != any)
            query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restriction[1])
                    + " AND to_id = " + std::to_string(restriction[2]) + ";";
        auto result = client.exec(query);
        return result[0][0].as<TLink>();
    }

    std::vector<std::array<TLink, 3>> Each(const LinkType& restrictions)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string_view query {};
        if (restrictions[0] == any && restrictions[1] == any && restrictions[2] == any)
            query = "SELECT * FROM Links;";
        else if (restrictions[0] != any && restrictions[1] == any && restrictions[2] == any)
            query = "SELECT * FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
        else if (restrictions[0] == any && restrictions[1] != any && restrictions[2] == any)
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + ";";
        else if (restrictions[0] == any && restrictions[1] == any && restrictions[2] != any)
            query = "SELECT * FROM Links WHERE to_id = " + std::to_string(restrictions[2]) + ";";
        else if (restrictions[0] == any && restrictions[1] != any && restrictions[2] != any)
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1])
                    + " AND to_id = " + std::to_string(restrictions[2]) + ";";
        auto result = client.exec(query);
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
