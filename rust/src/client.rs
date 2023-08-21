use crate::transaction::Transaction;
use doublets::{
    data::{Error, Flow, LinkType, LinksConstants, ReadHandler, WriteHandler},
    Doublets, Link, Links,
};
use std::error;
use tokio::runtime::Runtime;
use tokio_postgres as postgres;

pub struct Client<T: LinkType> {
    client: postgres::Client,
    constants: LinksConstants<T>,
    runtime: Runtime,
}

impl<T: LinkType> Client<T> {
    pub async fn new(client: postgres::Client) -> Result<Self, Box<dyn error::Error>> {
        client
            .query(
                "CREATE TABLE IF NOT EXISTS Links (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, from_id bigint, to_id bigint);",
                &[],
            )
            .await?;
        client
            .query(
                "CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);",
                &[],
            )
            .await?;
        client
            .query(
                "CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);",
                &[],
            )
            .await?;
        Ok(Self {
            client,
            constants: LinksConstants::<T>::new(),
            runtime: Runtime::new()?,
        })
    }

    pub fn transaction(&mut self) -> Result<Transaction<'_, T>, postgres::Error> {
        Ok(Transaction::new(
            self.runtime.block_on(self.client.transaction())?,
            &self.runtime,
        ))
    }

    pub(crate) fn drop_table(self) -> Result<(), Box<dyn error::Error>> {
        self.runtime
            .block_on(self.client.query("DROP TABLE Links;", &[]))?;
        Ok(())
    }
}

impl<T: LinkType> Links<T> for Client<T> {
    fn constants(&self) -> &LinksConstants<T> {
        &self.constants
    }

    fn count_links(&self, query: &[T]) -> T {
        let any = self.constants.any;
        if query.is_empty() {
            let result = self
                .runtime
                .block_on(self.client.query("SELECT COUNT(*) FROM Links;", &[]))
                .unwrap();
            let row = &result[0];
            row.get::<_, i64>(0).try_into().unwrap()
        } else if query.len() == 1 {
            if query[0] == any {
                self.count_links(&[])
            } else {
                let result = self
                    .runtime
                    .block_on(self.client.query(
                        format!("SELECT COUNT(*) FROM Links WHERE id = {};", query[0]).as_str(),
                        &[],
                    ))
                    .unwrap();
                result[0].get::<_, i64>(0).try_into().unwrap()
            }
        } else if query.len() == 3 {
            let id = if query[0] == any {
                String::new()
            } else {
                format!("id = {} AND ", query[0])
            };
            let source = if query[1] == any {
                String::new()
            } else {
                format!("from_id = {} AND", query[1])
            };
            let target = if query[2] == any {
                String::from("true;")
            } else {
                format!("to_id = {};", query[2])
            };
            let statement = format!(
                "SELECT COUNT(*) FROM Links WHERE {}{}{}",
                id, source, target
            );
            let result = self
                .runtime
                .block_on(self.client.query(&statement, &[]))
                .unwrap();
            result[0].get::<_, i64>(0).try_into().unwrap()
        } else {
            panic!("Constraints violation: size of query neither 1 nor 3")
        }
    }

    fn create_links(&mut self, _query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        let result = self
            .runtime
            .block_on(self.client.query(
                "INSERT INTO Links(to_id, from_id) VALUES (0, 0) RETURNING id;",
                &[],
            ))
            .unwrap();
        Ok(handler(
            Link::nothing(),
            Link::new(
                result[0].get::<_, i64>(0).try_into().unwrap(),
                T::ZERO,
                T::ZERO,
            ),
        ))
    }

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Flow {
        let any = self.constants.any;
        if query.is_empty() {
            let result = self
                .runtime
                .block_on(self.client.query("SELECT * FROM Links;", &[]))
                .unwrap();
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
                    .runtime
                    .block_on(self.client.query(
                        &format!("SELECT * FROM Links WHERE id = {};", query[0]),
                        &[],
                    ))
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
            let id = if query[0] == any {
                String::new()
            } else {
                format!("id = {} AND ", query[0])
            };
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
            let result = self
                .runtime
                .block_on(self.client.query(statement, &[]))
                .unwrap();
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
        let old_links = self
            .runtime
            .block_on(
                self.client
                    .query(&format!("SELECT * FROM Links WHERE id = {id};"), &[]),
            )
            .unwrap();
        let (old_source, old_target) = (
            old_links[0].get::<_, i64>(1).try_into().unwrap(),
            old_links[0].get::<_, i64>(2).try_into().unwrap(),
        );
        let query =
            format!("UPDATE Links SET from_id = {source}, to_id = {target} WHERE id = {id};");
        self.runtime
            .block_on(self.client.query(&query, &[]))
            .unwrap();
        Ok(handler(
            Link::new(id, old_source, old_target),
            Link::new(id, source, target),
        ))
    }

    fn delete_links(&mut self, query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        let runtime = Runtime::new()?;
        let id = query[0];
        let result = runtime
            .block_on(self.client.query(
                &format!("DELETE FROM Links WHERE id = {id} RETURNING from_id, to_id"),
                &[],
            ))
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

impl<T: LinkType> Doublets<T> for Client<T> {
    fn get_link(&self, index: T) -> Option<Link<T>> {
        let result = self.runtime.block_on(
            self.client
                .query(&format!("SELECT * FROM Links WHERE id = {index}"), &[]),
        );
        match result {
            Ok(rows) => {
                let row = &rows[0];
                Some(Link::new(
                    row.get::<_, i64>(0).try_into().unwrap(),
                    row.get::<_, i64>(1).try_into().unwrap(),
                    row.get::<_, i64>(2).try_into().unwrap(),
                ))
            }
            Err(_) => None,
        }
    }
}
