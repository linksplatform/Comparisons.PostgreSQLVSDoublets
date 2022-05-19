#include <array>
#include <vector>

#include <pqxx/pqxx>
#include <Platform.Data.h>

template<typename TLink>
struct LinksPSQL
{
    using LinkType = Platform::Data::LinksOptions<>::LinkType;

    explicit LinksPSQL(const std::string dbopts) : opts(dbopts)
    {
        query = "CREATE TABLE IF NOT EXISTS Links(id bigint, from_id bigint, to_id bigint);";
        line.retrieve(line.insert(query));
        GetIndex();
    }

    ~LinksPSQL()
    {
        line.complete();
        transaction.commit();
        connection.close();
    }

    bool Exists(auto&& Index)
    {
        auto result = line.retrieve(line.insert("SELECT * FROM Links WHERE id = " + std::to_string(Index) + ";"));
        return result[0][0].c_str() != "";
    }

    void Create(const LinkType& substitution)
    {
        line.insert("INSERT INTO Links VALUES (" + std::to_string(++index) + ", "
        + std::to_string(substitution[0]) + ", " + std::to_string(substitution[1]) + ");");
    }

    void Update(const LinkType& restrictions, const LinkType& substitution)
    {
        if (std::size(restrictions) == 1 || std::size(restrictions) == 3)
        {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE id = " + std::to_string(restrictions[0]) + ";";
        }
        if (std::size(restrictions) == 2)
        {
            query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                    + std::to_string(substitution[1]) + " WHERE from_id = " + std::to_string(restrictions[0])
                    + "AND to_id = " + std::to_string(restrictions[1]) + ";";
        }
        line.insert(query);
    }

    void Delete(const LinkType& restrictions)
    {
        if (!this->Exists(restrictions[0]))
            std::cout << "You can`t delete non-existent link.";
        else
        {
            query = "DELETE FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
            line.insert(query);
        }
    }

    void Complete() { line.complete(); }

    std::uint64_t Count(const LinkType& restrictions)
    {
        using namespace Platform::Data;
        LinksConstants<TLink> constants;
        if (restrictions[0] == constants.Any && restrictions[1] == constants.Any && restrictions[2] == constants.Any)
            query = "SELECT COUNT(*) FROM Links;";
        else if (restrictions[0] != constants.Any && restrictions[1] == constants.Any && restrictions[2] == constants.Any)
            query = "SELECT COUNT(*) FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
        else if (restrictions[0] == constants.Any && restrictions[1] != constants.Any && restrictions[2] == constants.Any)
            query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + ";";
        else if (restrictions[0] == constants.Any && restrictions[1] == constants.Any && restrictions[2] != constants.Any)
            query = "SELECT COUNT(*) FROM Links WHERE to_id = " + std::to_string(restrictions[2]) + ";";
        else if (restrictions[0] == constants.Any && restrictions[1] != constants.Any && restrictions[2] != constants.Any)
            query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restrictions[1])
                    + "AND to_id = " + std::to_string(restrictions[2]) + ";";
        auto result = line.retrieve(line.insert(query));
        return result[0][0].as<std::uint64_t>();
    }

    std::vector<std::array<TLink, 3>> Each(const LinkType& restrictions)
    {
        using namespace Platform::Data;
        LinksConstants<TLink> constants;
        if (restrictions[0] == constants.Any && restrictions[1] == constants.Any && restrictions[2] == constants.Any)
            query = "SELECT * FROM Links;";
        else if (restrictions[0] != constants.Any && restrictions[1] == constants.Any && restrictions[2] == constants.Any)
            query = "SELECT * FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
        else if (restrictions[0] == constants.Any && restrictions[1] != constants.Any && restrictions[2] == constants.Any)
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + ";";
        else if (restrictions[0] == constants.Any && restrictions[1] == constants.Any && restrictions[2] != constants.Any)
            query = "SELECT * FROM Links WHERE to_id = " + std::to_string(restrictions[2]) + ";";
        else if (restrictions[0] == constants.Any && restrictions[1] != constants.Any && restrictions[2] != constants.Any)
            query = "SELECT * FROM Links WHERE from_id = " + std::to_string(restrictions[1]) + "AND to_id = "
                    + std::to_string(restrictions[2]) + ";";
        auto result = line.retrieve(line.insert(query));
        std::vector<std::array<TLink, 3>> links {};
        for(int i{}; i<result.size(); i++)
        {
            std::array<TLink, 3> link {};
            for(int j{}; j<3; j++)
            {
                link[j] = result[i][j].as<TLink>();
            }
            links.push_back(link);
        }
        return links;
    }

    private: std::string opts{};
    private: std::string query{};
    private: std::uint64_t index{};

    private: void GetIndex()
    {
        auto r = line.retrieve(line.insert("SELECT * FROM Links;"));
        index = r.size();
    }

    private: pqxx::connection connection {opts};
    private: pqxx::work transaction {connection};
    private: pqxx::pipeline line {transaction};
};
