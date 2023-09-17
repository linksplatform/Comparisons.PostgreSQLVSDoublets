use {
    crate::{transaction::Transaction, Exclusive, Result, Sql},
    doublets::{
        data::{Error, Flow, LinkType, LinksConstants, ReadHandler, WriteHandler},
        Doublets, Link, Links,
    },
};

pub struct Client<T: LinkType> {
    client: postgres::Client,
    constants: LinksConstants<T>,
}

impl<T: LinkType> Client<T> {
    pub fn new(mut client: postgres::Client) -> Result<Self> {
        client.query(
            "CREATE TABLE IF NOT EXISTS Links (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, \
            from_id bigint, to_id bigint);",
            &[],
        )?;
        client.query("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);", &[])?;
        client.query("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);", &[])?;
        Ok(Self { client, constants: LinksConstants::new() })
    }

    pub fn transaction(&mut self) -> Result<Transaction<'_, T>> {
        Ok(Transaction::new(self.client.transaction()?))
    }
}

impl<T: LinkType> Sql for Client<T> {
    fn create_table(&mut self) -> Result<()> {
        self.client.query(
            "CREATE TABLE IF NOT EXISTS Links (id bigint\
                     GENERATED ALWAYS AS IDENTITY PRIMARY KEY, from_id bigint, to_id bigint);",
            &[],
        )?;
        self.client
            .query("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);", &[])?;
        self.client.query("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);", &[])?;
        Ok(())
    }

    fn drop_table(&mut self) -> Result<()> {
        self.client.query("DROP TABLE Links;", &[])?;
        Ok(())
    }
}

impl<'c, T: LinkType> Links<T> for Exclusive<Client<T>> {
    fn constants(&self) -> &LinksConstants<T> {
        &self.constants
    }

    fn count_links(&self, query: &[T]) -> T {
        let any = self.constants.any;
        if query.is_empty() {
            let result = self.get().client.query("SELECT COUNT(*) FROM Links;", &[]).unwrap();
            let row = &result[0];
            row.get::<_, i64>(0).try_into().unwrap()
        } else if query.len() == 1 {
            if query[0] == any {
                self.count_links(&[])
            } else {
                let result = self
                    .get()
                    .client
                    .query("SELECT COUNT(*) FROM Links WHERE id = $1;", &[&query[0].as_i64()])
                    .unwrap();
                result[0].get::<_, i64>(0).try_into().unwrap()
            }
        } else if query.len() == 3 {
            let id =
                if query[0] == any { String::new() } else { format!("id = {} AND ", query[0]) };
            let source =
                if query[1] == any { String::new() } else { format!("from_id = {} AND", query[1]) };
            let target = if query[2] == any {
                String::from("true;")
            } else {
                format!("to_id = {};", query[2])
            };
            let statement = format!("SELECT COUNT(*) FROM Links WHERE {}{}{}", id, source, target);
            let result = self.get().client.query(&statement, &[]).unwrap();
            result[0].get::<_, i64>(0).try_into().unwrap()
        } else {
            panic!("Constraints violation: size of query neither 1 nor 3")
        }
    }

    fn create_links(&mut self, _query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        let result = self
            .client
            .query("INSERT INTO Links(to_id, from_id) VALUES (0, 0) RETURNING id;", &[])
            .unwrap();
        Ok(handler(
            Link::nothing(),
            Link::new(result[0].get::<_, i64>(0).try_into().unwrap(), T::ZERO, T::ZERO),
        ))
    }

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Flow {
        let any = self.constants.any;
        if query.is_empty() {
            let result = self.get().client.query("SELECT * FROM Links;", &[]).unwrap();
            for row in result {
                handler(Link::new(
                    row.get::<_, i64>(0).try_into().unwrap(),
                    row.get::<_, i64>(1).try_into().unwrap(),
                    row.get::<_, i64>(2).try_into().unwrap(),
                ))?
            }
            Flow::Continue
        } else if query.len() == 1 {
            if query[0] == any {
                self.each_links(&[], handler)
            } else {
                let result = self
                    .get()
                    .client
                    .query("SELECT * FROM Links WHERE id = %1;", &[&query[0].as_i64()])
                    .unwrap();
                for row in result {
                    handler(Link::new(
                        row.get::<_, i64>(0).try_into().unwrap(),
                        row.get::<_, i64>(1).try_into().unwrap(),
                        row.get::<_, i64>(2).try_into().unwrap(),
                    ))?
                }
                Flow::Continue
            }
        } else if query.len() == 3 {
            let id =
                if query[0] == any { String::new() } else { format!("id = {} AND ", query[0]) };
            let source = if query[1] == any {
                String::new()
            } else {
                format!("from_id = {} AND ", query[1])
            };
            let target = if query[2] == any {
                String::from("true;")
            } else {
                format!("to_id = {};", query[2])
            };
            let statement = &format!("SELECT * FROM Links WHERE {id}{source}{target}");
            let result = self.get().client.query(statement, &[]).unwrap();
            for row in result {
                handler(Link::new(
                    row.get::<_, i64>(0).try_into().unwrap(),
                    row.get::<_, i64>(1).try_into().unwrap(),
                    row.get::<_, i64>(2).try_into().unwrap(),
                ))?
            }
            Flow::Continue
        } else {
            panic!("Constraints violation: size of query neither 1 nor 3")
        }
    }

    fn update_links(
        &mut self,
        query: &[T],
        change: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Error<T>> {
        let id = query[0];
        let source = change[1];
        let target = change[2];
        let old_links =
            self.client.query("SELECT * FROM Links WHERE id = $1;", &[&id.as_i64()]).unwrap();
        let (old_source, old_target) = (
            old_links[0].get::<_, i64>(1).try_into().unwrap(),
            old_links[0].get::<_, i64>(2).try_into().unwrap(),
        );
        let _ = self.client.query(
            "UPDATE Links SET from_id = $1, to_id = $2 WHERE id = $3;",
            &[&source.as_i64(), &target.as_i64(), &id.as_i64()],
        );
        Ok(handler(Link::new(id, old_source, old_target), Link::new(id, source, target)))
    }

    fn delete_links(&mut self, query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        let id = query[0];
        let result = self
            .client
            .query("DELETE FROM Links WHERE id = $1 RETURNING from_id, to_id", &[&id.as_i64()])
            .unwrap();
        let row = if result.is_empty() {
            return Err(Error::<T>::NotExists(id));
        } else {
            &result[0]
        };
        Ok(handler(
            Link::new(
                id,
                row.get::<_, i64>(0).try_into().unwrap(),
                row.get::<_, i64>(1).try_into().unwrap(),
            ),
            Link::nothing(),
        ))
    }
}

impl<'c, T: LinkType> Doublets<T> for Exclusive<Client<T>> {
    fn get_link(&self, index: T) -> Option<Link<T>> {
        let result =
            self.get().client.query("SELECT * FROM Links WHERE id = $1", &[&index.as_i64()]);
        match result {
            Ok(rows) => {
                let row = &rows[0];
                if row.is_empty() {
                    None
                } else {
                    Some(Link::new(
                        row.get::<_, i64>(0).try_into().unwrap(),
                        row.get::<_, i64>(1).try_into().unwrap(),
                        row.get::<_, i64>(2).try_into().unwrap(),
                    ))
                }
            }
            Err(_) => None,
        }
    }
}
