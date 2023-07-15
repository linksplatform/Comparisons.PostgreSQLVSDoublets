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
        using TLinkAddress = typename TLinksOptions::LinkAddressType;
        
        explicit PSQLBase(const std::string& options): connection(options)
        {
            execute(
                "CREATE TABLE IF NOT EXISTS Links ("
                    "id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, "
                    "from_id bigint, "
                    "to_id bigint"
                ");"
            );
            execute("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);");
            execute("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);");
        }
        
        ~PSQLBase() {
            if constexpr (std::same_as<TExecutor, pqxx::transaction<>>) {
                executor.commit();
            }
            connection.close();
        }
        
    private:
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
        
        [[nodiscard]] std::string escape(std::string query) const {
            return executor.esc(query);
        }
        
        TExecutor executor {connection};
        pqxx::connection connection;
    };
}