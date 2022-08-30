#include <array>
#include <vector>

#include <pqxx/pqxx>
#include <Platform.Data.h>

template<typename TLink>
struct Transaction 
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;

    explicit Transaction(std::string opts): connection(opts)
    {
        index = transaction.exec("SELECT * FROM Links;").size();
    }

    ~Transaction() {
        transaction.commit();
        connection.close();
    };

    void Create(const LinkType& substitution)
    {
        transaction.exec("INSERT INTO Links VALUES (" + std::to_string(++index) + ", "
                         + std::to_string(substitution[0]) + ", " + std::to_string(substitution[1]) + ");");
    }

    void Update(const LinkType& restriction, const LinkType& substitution)
    {
        std::string query{};
        if (std::size(restriction) == 1 || std::size(restriction) == 3) {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE id = " + std::to_string(restriction[0]) + ";";
        } else if (std::size(restriction) == 2) {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE from_id = " + std::to_string(restriction[0])
                    + " AND to_id = " + std::to_string(restriction[1]) + ";";
        }
        transaction.exec(query);
    }

    void Delete(const LinkType& restriction)
    {
        std::string query{};
        if (std::size(restriction) == 1 || std::size(restriction) == 3) {
            query = "DELETE FROM Links WHERE id = " + std::to_string(restriction[0]) + ";";
        } else if (std::size(restriction) == 2) {
            query = "DELETE FROM Links WHERE from_id = " + std::to_string(restriction[0])
                    + " AND to_id = " + std::to_string(restriction[1]) + ";";
        }
        transaction.exec(query);
    }

    TLink Count(const LinkType& restriction)
    {
        using namespace Platform::Data;
        auto any = LinksConstants<TLink>().Any;
        std::string query{};
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
        auto result = transaction.exec(query);
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
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1])
                    + " AND to_id = " + std::to_string(restrictions[2]) + ";";
        auto result = transaction.exec(query);
        std::vector<std::array<TLink, 3>> links{};
        for (int i{}; i < result.size(); ++i) {
            std::array<TLink, 3> link{};
            for (int j{}; j < 3; j++) { link[j] = result[i][j].as<TLink>(); }
            links.push_back(link);
        }
        return links;
    }

    private: TLink index{};

    private: pqxx::connection connection;
    private: pqxx::transaction<> transaction{connection};
};
