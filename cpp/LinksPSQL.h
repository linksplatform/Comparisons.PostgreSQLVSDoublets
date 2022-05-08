#include <pqxx/pqxx>

namespace Platform::Data::Doublets
{
    template<typename TLink>
    struct LinksPSQL: public ILinks<LinksPSQL<TLink>, TLink>
    {
        explicit LinksPSQL(const std::string dbopts) : opts(std::move(dbopts))
        {
            query = "CREATE TABLE IF NOT EXISTS Links(id bigint, from_id bigint, to_id bigint);";
            line.insert(query);
        }
        
        ~LinksPSQL()
        {
            line.complete();
            trans.commit();
            conn.close();
        }
        
        auto Exists(auto&& Index) -> bool
        {
            auto result = line.retrieve(line.insert("SELECT * FROM Links WHERE id = " + std::to_string(Index) + ";"));
            return result[0][0].c_str() != "";
        }
        
        auto Create(Interfaces::CArray auto&& substitution) -> void
        {
            line.insert("INSERT INTO Links VALUES (" + std::to_string(++_index) + ", "
            + std::to_string(substitution[0]) + ", " + std::to_string(substitution[1]) + ");");
        }
        
        auto Update(Interfaces::CArray auto&& restrictions, Interfaces::CArray auto&& substitution) -> void
        {
            if (std::size(restrictions)==1 || std::size(restrictions)==3)
            {
                query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                + std::to_string(substitution[1]) + " WHERE id = " + std::to_string(restrictions[0]) + ";";  
            }
            if(std::size(restrictions)==2)
            {
                query = "UPDATE Links SET from_id = " + std::to_string(substitution[0]) + ", to_id = "
                + std::to_string(substitution[1]) + " WHERE from_id = " + std::to_string(restrictions[0])
                + "AND to_id = " + std::to_string(restrictions[1]) + ";";
            }
            line.insert(query);
        }
        
        auto Delete(Interfaces::CArray auto&& restrictions) -> void
        {
            if(!this->Exists(restrictions[0]))
                std::cout<<"You can`t delete non-existent link.";
            else
            {
                query = "DELETE FROM Links WHERE id = " + std::to_string(restrictions[0]) + ";";
                line.insert(query);
            }
        }
        
        auto Count(Interfaces::CArray auto&& restrictions) -> int
        {
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
                query = "SELECT COUNT(*) FROM Links WHERE from_id = " + std::to_string(restrictions[1]) +
                        "AND to_id = " + std::to_string(restrictions[2]) + ";";
            auto result = line.retrieve(line.insert(query));
            return result[0][0].as<int>();
        }
        
        auto Each(Interfaces::CArray auto&& restrictions) -> std::vector<Link<TLink>>
        {
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
            std::vector<Link<TLink>> links {};
            Link<TLink> link {};
            for(int i{}; i<result.size(); i++)
            {
                for(int j{}; j<3; j++)
                {
                    link[j] = result[i][j].as<TLink>();
                    links.push_back(link);
                }
            }
            return links;
        }

        private: std::string opts{};
        private: std::string query{};
        private: int _index{};
        
        private: constexpr void GetIndex()
        {
            pqxx::work trans2 {conn};
            pqxx::result r {trans2.exec("SELECT * FROM Links;")};
            trans2.commit();
            _index = r.size();
        }
        
        private: pqxx::connection conn {opts};
        private: pqxx::work trans {conn};
        private: pqxx::pipeline line {trans};
    };
}