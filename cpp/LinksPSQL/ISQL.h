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
        static constexpr LinksConstants<TLinkAddress> Constants = TLinksOptions::Constants;
        
        virtual TResult execute(std::string query) const = 0;
        virtual TLinkAddress cast(TElement element) const = 0;
        virtual TRow first_row(TResult result) const = 0;
        virtual TElement identity(TRow row) const = 0;
        virtual TElement from_id(TRow row) const = 0;
        virtual TElement to_id(TRow row) const = 0;
        virtual bool empty(TResult) const = 0;
        virtual TLinkAddress foreach(TResult& result, ReadHandlerType handler) const = 0;
        [[nodiscard]] virtual std::string escape(std::string query) const = 0;
        
        
        TLinkAddress Count(const std::vector<TLinkAddress>& restriction) const {
            auto any {Constants.Any};
            std::string query{"SELECT COUNT(*) FROM Links WHERE "};
            if (std::size(restriction) == 0) {
                query.append("true;");
            } else if (std::size(restriction) == 1) {
                std::string id {restriction[0] == any ? "true;" : "id = " + escape(std::to_string(restriction[0])) + ";"};
                query.append(id);
            } else if (std::size(restriction) == 3) {
                std::string id {restriction[0] == any ? "" : "id = " + escape(std::to_string(restriction[0])) + " AND "};
                std::string source {restriction[1] == any ? "" : "from_id = " + escape(std::to_string(restriction[1])) + " AND "};
                std::string target {restriction[2] == any ? "true;" : "to_id = " + escape(std::to_string(restriction[2])) + ";"};
                query.append(id + source + target);
            } else {
                throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
            }
            TResult result {execute(query)};
            return cast(identity(first_row(result)));
        }

        TLinkAddress Each(const std::vector<TLinkAddress>& restriction, const ReadHandlerType& handler) const {
            auto any {Constants.Any};
            auto $continue {LinksConstants<TLinkAddress>{}.Continue};
            std::string query{"SELECT * FROM Links WHERE "};
            if (std::size(restriction) == 0) {
                query.append("true;");
            } else if (std::size(restriction) == 1) {
                std::string id {restriction[0] == any ? "true;" : "id = " + escape(std::to_string(restriction[0])) + ";"};
                query.append(id);
            } else if (std::size(restriction) == 3) {
                std::string id {restriction[0] == any ? "" : "id = " + escape(std::to_string(restriction[0])) + " AND "};
                std::string source {restriction[1] == any ? "" : "from_id = " + escape(std::to_string(restriction[1])) + " AND "};
                std::string target {restriction[2] == any ? "true;" : "to_id = " + escape(std::to_string(restriction[2])) + ";"};
                query.append(id + source + target);
            } else {
                throw std::invalid_argument("Constraints violation: size of restriction neither 1 nor 3.");
            }
            TResult result {execute(query)};
            if (empty(result)) {
                return $continue;
            }
            return foreach(result, handler);
        }
        
        TLinkAddress Create(const std::vector<TLinkAddress>& substitution, const WriteHandlerType& handler) {
            TResult result {execute("INSERT INTO Links(from_id, to_id) VALUES (0, 0) RETURNING id;")};
            return handler({}, {cast(identity(first_row(result))), 0, 0});
        }

        TLinkAddress Update(const std::vector<TLinkAddress>& restriction, const std::vector<TLinkAddress>& substitution, const WriteHandlerType& handler) {
            auto any {Constants.Any};
            std::string query {
                "UPDATE Links SET from_id = " + escape(std::to_string(substitution[1]))
                + ", to_id = " + escape(std::to_string(substitution[2])) + " WHERE "
            };
            if (std::size(restriction) == 1) {
                std::string id {restriction[0] == any ? "true" : "id = " + escape(std::to_string(restriction[0]))};
                query.append(id);
            } else if (std::size(restriction) == 3) {
                std::string id {restriction[0] == any ? "" : "id = " + escape(std::to_string(restriction[0])) + " AND "};
                std::string source {restriction[1] == any ? "" : "from_id = " + escape(std::to_string(restriction[1])) + " AND "};
                std::string target {restriction[2] == any ? "true" : "to_id = " + escape(std::to_string(restriction[2]))};
                query.append(id + source + target);
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
            auto any {Constants.Any};
            std::string query {"DELETE FROM Links WHERE "};
            if (std::size(restriction) == 1) {
                std::string id {restriction[0] == any ? "true" : "id = " + escape(std::to_string(restriction[0]))};
                query.append(id);
            } else if (std::size(restriction) == 3) {
                std::string id = restriction[0] == any ? "" : "id = " + escape(std::to_string(restriction[0])) + " AND ";
                std::string source = restriction[1] == any ? "" : "from_id = " + escape(std::to_string(restriction[1])) + " AND ";
                std::string target = restriction[2] == any ? "true" : "to_id = " + escape(std::to_string(restriction[2]));
                query.append(id + source + target);
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
