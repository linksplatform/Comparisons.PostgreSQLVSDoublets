namespace PostgreSQL
{
    using namespace Platform::Data::Doublets::Sql;
    template <
        typename TLinksOptions,
        typename TExecutor, 
        typename TResult = pqxx::result, 
        typename TRow = pqxx::row, 
        typename TElement = pqxx::field
    >
    struct PSQLBase: public ISQL<TLinksOptions, TResult, TRow, TElement>
    {
        explicit PSQLBase(std::string option): connection(option)
        {
            executor.exec(
                "CREATE TABLE IF NOT EXISTS Links ("
                    "id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, "
                    "from_id bigint, "
                    "to_id bigint"
                ");"
            );
            executor.exec("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);");
            executor.exec("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);");
        }
        
        virtual ~PSQLBase() {
            if constexpr (std::same_as<TExecutor, pqxx::transaction<>>) {
                executor.commit();
            }
            connection.close();
        }
        
    private:
        using TLinkAddress = typename TLinksOptions::LinkAddressType;
        using ReadHandlerType = typename TLinksOptions::ReadHandlerType;

        TResult execute(std::string query) const {
            return executor.exec(query);
        };
        
        TLinkAddress cast(TElement element) const {
            return element.template as<TLinkAddress>();
        };

        TRow first_row(TResult result) const {
            return result[0];
        }
        
        TElement identity(TRow row) const {
            return row[0];
        }
        
        TElement from_id(TRow row) const {
            return row[1];
        }
        
        TElement to_id(TRow row) const {
            return row[2];
        };

        bool empty(TResult result) const {
            return result.empty();
        }

        TLinkAddress foreach(TResult& result, ReadHandlerType handler) const {
            auto $break = TLinksOptions::Constants.Break;
            auto $continue = TLinksOptions::Constants.Continue;
            for (auto row: result) {
                if (handler({cast(row[0]), cast(row[1]), cast(row[2])}) == $break) {
                    return $break;
                }
            }
            return $continue;
        }

        [[nodiscard]] std::string escape(std::string query) const {
            return executor.esc(query);
        }

        pqxx::connection connection;
        mutable TExecutor executor {connection};
    };
}