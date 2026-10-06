use {
    crate::{Exclusive, Result, Sql, as_i64},
    doublets::{
        Doublets, Error, Link, Links,
        data::{Flow, LinkReference, LinksConstants, ReadHandler, WriteHandler},
    },
};

pub struct Transaction<'a, T: LinkReference> {
    transaction: std::sync::Mutex<postgres::Transaction<'a>>,
    constants: LinksConstants<T>,
}

impl<'a, T: LinkReference> Transaction<'a, T> {
    pub fn new(transaction: postgres::Transaction<'a>) -> Self {
        Self {
            transaction: std::sync::Mutex::new(transaction),
            constants: LinksConstants::<T>::new(),
        }
    }

    pub fn commit(&mut self) -> Result<()> {
        self.transaction.lock().unwrap().execute("COMMIT;", &[])?;
        self.transaction.lock().unwrap().execute("BEGIN;", &[])?;
        Ok(())
    }
}

impl<'a, T: LinkReference> Sql for Transaction<'a, T> {
    fn create_table(&mut self) -> Result<()> {
        self.transaction.lock().unwrap().query(
            "CREATE TABLE IF NOT EXISTS Links (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, from_id bigint, to_id bigint);",
            &[],
        )?;
        self.transaction
            .lock()
            .unwrap()
            .query("CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);", &[])?;
        self.transaction
            .lock()
            .unwrap()
            .query("CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);", &[])?;
        Ok(())
    }

    fn drop_table(&mut self) -> Result<()> {
        self.transaction.lock().unwrap().query("DROP TABLE Links;", &[])?;
        self.commit()
    }
}

impl<T: LinkReference> Links<T> for Exclusive<Transaction<'_, T>> {
    fn constants(&self) -> &LinksConstants<T> {
        &self.constants
    }

    fn count_links(&self, query: &[T]) -> T {
        let any = self.constants.any;
        if query.is_empty() {
            let result = self
                .get()
                .transaction
                .lock()
                .unwrap()
                .query("SELECT COUNT(*) FROM Links;", &[])
                .unwrap();
            let row = &result[0];
            row.get::<_, i64>(0).try_into().unwrap()
        } else if query.len() == 1 {
            if query[0] == any {
                self.count_links(&[])
            } else {
                let result = self
                    .get()
                    .transaction
                    .lock()
                    .unwrap()
                    .query("SELECT COUNT(*) FROM Links WHERE id = $1;", &[&as_i64(query[0])])
                    .unwrap();
                result[0].get::<_, i64>(0).try_into().unwrap()
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
            let statement = format!("SELECT COUNT(*) FROM Links WHERE {}{}{}", id, source, target);
            let result = self.get().transaction.lock().unwrap().query(&statement, &[]).unwrap();
            result[0].get::<_, i64>(0).try_into().unwrap()
        } else {
            panic!("Constraints violation: size of query neither 1 nor 3")
        }
    }

    fn create_links(&mut self, _query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        let result = self
            .transaction
            .lock()
            .unwrap()
            .query("INSERT INTO Links(to_id, from_id) VALUES (0, 0) RETURNING id;", &[])
            .unwrap();
        Ok(handler(
            Link::nothing(),
            Link::new(
                result[0].get::<_, i64>(0).try_into().unwrap(),
                T::from_byte(0),
                T::from_byte(0),
            ),
        ))
    }

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Flow {
        let any = self.constants.any;
        if query.is_empty() {
            let result =
                self.get().transaction.lock().unwrap().query("SELECT * FROM Links;", &[]).unwrap();
            for row in result {
                if handler(Link::new(
                    row.get::<_, i64>(0).try_into().unwrap(),
                    row.get::<_, i64>(1).try_into().unwrap(),
                    row.get::<_, i64>(2).try_into().unwrap(),
                ))
                .is_break()
                {
                    return Flow::Break;
                }
            }
            Flow::Continue
        } else if query.len() == 1 {
            if query[0] == any {
                self.each_links(&[], handler)
            } else {
                let result = self
                    .get()
                    .transaction
                    .lock()
                    .unwrap()
                    .query("SELECT * FROM Links WHERE id = $1;", &[&as_i64(query[0])])
                    .unwrap();
                for row in result {
                    if handler(Link::new(
                        row.get::<_, i64>(0).try_into().unwrap(),
                        row.get::<_, i64>(1).try_into().unwrap(),
                        row.get::<_, i64>(2).try_into().unwrap(),
                    ))
                    .is_break()
                    {
                        return Flow::Break;
                    }
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
            let result = self.get().transaction.lock().unwrap().query(statement, &[]).unwrap();
            for row in result {
                if handler(Link::new(
                    row.get::<_, i64>(0).try_into().unwrap(),
                    row.get::<_, i64>(1).try_into().unwrap(),
                    row.get::<_, i64>(2).try_into().unwrap(),
                ))
                .is_break()
                {
                    return Flow::Break;
                }
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
        let old_links = self
            .transaction
            .lock()
            .unwrap()
            .query("SELECT * FROM Links WHERE id = $1;", &[&as_i64(id)])
            .unwrap();
        let (old_source, old_target) = (
            old_links[0].get::<_, i64>(1).try_into().unwrap(),
            old_links[0].get::<_, i64>(2).try_into().unwrap(),
        );
        self.transaction
            .lock()
            .unwrap()
            .query(
                "UPDATE Links SET from_id = $1, to_id = $2 WHERE id = $3;",
                &[&as_i64(source), &as_i64(target), &as_i64(id)],
            )
            .unwrap();
        Ok(handler(Link::new(id, old_source, old_target), Link::new(id, source, target)))
    }

    fn delete_links(&mut self, query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        let id = query[0];
        let result = self
            .transaction
            .lock()
            .unwrap()
            .query("DELETE FROM Links WHERE id = $1 RETURNING from_id, to_id", &[&as_i64(id)])
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

impl<T: LinkReference> Doublets<T> for Exclusive<Transaction<'_, T>> {
    fn get_link(&self, index: T) -> Option<Link<T>> {
        let result = self
            .get()
            .transaction
            .lock()
            .unwrap()
            .query("SELECT * FROM Links WHERE id = $1", &[&as_i64(index)]);
        let rows = result.ok()?;
        let row = rows.first()?;
        Some(Link::new(
            row.get::<_, i64>(0).try_into().unwrap(),
            row.get::<_, i64>(1).try_into().unwrap(),
            row.get::<_, i64>(2).try_into().unwrap(),
        ))
    }
}
