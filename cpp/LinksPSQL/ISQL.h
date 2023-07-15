namespace Platform::Data::Doublets::Sql 
{
    template <
        typename TLinksOptions,
        typename TResult, 
        typename TRow, 
        typename TElement
    > 
    struct ISQL: public ILinks<TLinksOptions> 
    {
        using base = Data::ILinks<TLinksOptions>;
        using TLinkAddress = base::LinkAddressType;
        using ReadHandlerType = base::ReadHandlerType;
        using WriteHandlerType = base::WriteHandlerType;
        static constexpr Data::LinksConstants<TLinkAddress> Constants {};
        
        virtual TResult execute(std::string query) const = 0;
        virtual TLinkAddress cast(TElement element) const = 0;
        virtual TRow first_row(TResult result) const = 0;
        virtual TElement identity(TRow row) const = 0;
        virtual TElement from_id(TRow row) const = 0;
        virtual TElement to_id(TRow row) const = 0;
        [[nodiscard]] virtual std::string escape(std::string query) const = 0;
        
        
        TLinkAddress Count(const std::vector<TLinkAddress>& restriction) const {
            auto any = LinksConstants<TLinkAddress>{}.Any;
            auto null = LinksConstants<TLinkAddress>{}.Null;
            std::string query = "SELECT COUNT(*) FROM Links WHERE ";
            std::string id = restriction[0] == any ? "" : "id = " + escape(std::to_string(restriction[0])) + " AND ";
            std::string source = restriction[1] == any ? "" : "from_id = " + escape(std::to_string(restriction[1])) + " AND ";
            std::string target = restriction[2] == any ? "true;" : "to_id = " + escape(std::to_string(restriction[2])) + ";";
            query.append(id + source + target);
            TResult result = execute(query);
            return cast(identity(first_row(result)));
        }

        TLinkAddress Each(const std::vector<TLinkAddress>& restriction, const ReadHandlerType& handler) const {
            auto any = LinksConstants<TLinkAddress>{}.Any;
            auto $continue = LinksConstants<TLinkAddress>{}.Continue;
            std::string query = "SELECT * FROM Links WHERE ";
            std::string id = restriction[0] == any ? "" : "id = " + escape(std::to_string(restriction[0])) + " AND ";
            std::string source = restriction[1] == any ? "" : "from_id = " + escape(std::to_string(restriction[1])) + " AND ";
            std::string target = restriction[2] == any ? "true;" : "to_id = " + escape(std::to_string(restriction[2])) + ";";
            query.append(id + source + target);
            TResult result = execute(query);
            if (result.empty()) {
                return $continue;
            }
            TRow row = first_row(result);
            return handler({cast(identity(row)), cast(from_id(row)), cast(to_id(row))});
        }
        
        TLinkAddress Create(const std::vector<TLinkAddress>& restriction, const WriteHandlerType& handler) {
            TResult result = execute("INSERT INTO Links(from_id, to_id) VALUES (0, 0) RETURNING id;");
            return handler({}, {cast(identity(first_row(result))), 0, 0});
        }

        TLinkAddress Update(const std::vector<TLinkAddress>& restriction, const std::vector<TLinkAddress>& substitution, const WriteHandlerType& handler) {
            std::string query {};
            if (std::size(restriction) == 1) {
                query = "UPDATE Links SET from_id = " + escape(std::to_string(substitution[1]))
                        + ", to_id = " + escape(std::to_string(substitution[2]))
                        + " WHERE id = " + escape(std::to_string(restriction[0]));
            } else if (std::size(restriction) == 3) {
                query = "UPDATE Links SET from_id = " + escape(std::to_string(substitution[1]))
                        + ", to_id = " + escape(std::to_string(substitution[2]))
                        + " WHERE from_id = " + escape(std::to_string(restriction[1]))
                        + " AND to_id = " + escape(std::to_string(restriction[2]));
            } else {
                throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
            }
            query.append(" RETURNING id;");
            TResult result_prev = execute("SELECT * FROM Links WHERE id = " + escape(std::to_string(restriction[0])) + ";");
            TResult result = execute(query);
            TRow before = first_row(result_prev);
            TRow after = first_row(result);
            return handler(
                {cast(identity(before)), cast(from_id(before)), cast(to_id(before))},
                {cast(identity(after)), substitution[1], substitution[2]}
            );
        }
        
        TLinkAddress Delete(const std::vector<TLinkAddress>& restriction, const WriteHandlerType& handler) {
            std::string query {};
            if (std::size(restriction) == 1) {
                query = "DELETE FROM Links WHERE id = " + escape(std::to_string(restriction[0]));
            } else if (std::size(restriction) == 3) {
                query = "DELETE FROM Links WHERE from_id = " + escape(std::to_string(restriction[1]))
                        + " AND to_id = " + escape(std::to_string(restriction[2]));
            } else {
                throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
            }
            query.append(" RETURNING id, from_id, to_id;");
            TResult result = execute(query);
            TRow before = first_row(result);
            return handler({cast(identity(before)), cast(from_id(before)), cast(to_id(before))}, {});
        }

        void Drop() {
            execute("DROP TABLE Links;");
        }
    };
}
