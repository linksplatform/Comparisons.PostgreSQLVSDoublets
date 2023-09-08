use {
    crate::Sql,
    doublets::{
        data::{Flow, LinkType, LinksConstants, ReadHandler, WriteHandler},
        Doublets, Error, Link, Links,
    },
    std::error,
    tokio::runtime::Runtime,
    tokio_postgres as postgres,
};

pub struct Transaction<'a, T: LinkType> {
    transaction: postgres::Transaction<'a>,
    runtime: &'a Runtime,
    constants: LinksConstants<T>,
}

impl<'a, T: LinkType> Transaction<'a, T> {
    pub fn new(transaction: postgres::Transaction<'a>, runtime: &'a Runtime) -> Transaction<'a, T> {
        Transaction {
            transaction,
            runtime,
            constants: LinksConstants::<T>::new(),
        }
    }

    pub fn commit(&self) -> Result<(), Box<dyn error::Error>> {
        self.runtime.block_on(async {
            self.transaction.execute("COMMIT;", &[]).await?;
            self.transaction.execute("BEGIN;", &[]).await?;
            Ok(())
        })
    }
}

impl<'a, T: LinkType> Sql for Transaction<'a, T> {
    fn create_table(&mut self) -> crate::Result<()> {
        self.runtime.block_on(async {
            self.transaction
                .query(
                    "CREATE TABLE IF NOT EXISTS Links (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, from_id bigint, to_id bigint);",
                    &[],
                )
                .await?;
            self.transaction
                .query(
                    "CREATE INDEX IF NOT EXISTS source ON Links USING btree(from_id);",
                    &[],
                )
                .await?;
            self.transaction
                .query(
                    "CREATE INDEX IF NOT EXISTS target ON Links USING btree(to_id);",
                    &[],
                )
                .await?;
            Ok(())
        })
    }

    fn drop_table(&mut self) -> crate::Result<()> {
        self.runtime.block_on(async {
            self.transaction.query("DROP TABLE Links;", &[]).await?;
            self.commit()
        })
    }
}

impl<T: LinkType> Links<T> for Transaction<'_, T> {
    fn constants(&self) -> &LinksConstants<T> {
        &self.constants
    }

    fn count_links(&self, query: &[T]) -> T {
        self.runtime.block_on(async {
            let any = self.constants.any;
            if query.is_empty() {
                let result = self
                    .transaction
                    .query("SELECT COUNT(*) FROM Links;", &[])
                    .await
                    .unwrap();
                let row = &result[0];
                row.get::<_, i64>(0).try_into().unwrap()
            } else if query.len() == 1 {
                if query[0] == any {
                    self.count_links(&[])
                } else {
                    let result = self
                        .transaction
                        .query(
                            "SELECT COUNT(*) FROM Links WHERE id = $1;",
                            &[&query[0].as_i64()],
                        )
                        .await
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
                    format!("from_id = {} AND ", query[1])
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
                let result = self.transaction.query(&statement, &[]).await.unwrap();
                result[0].get::<_, i64>(0).try_into().unwrap()
            } else {
                panic!("Constraints violation: size of query neither 1 nor 3")
            }
        })
    }

    fn create_links(&mut self, _query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        self.runtime.block_on(async {
            let result = self
                .transaction
                .query(
                    "INSERT INTO Links(to_id, from_id) VALUES (0, 0) RETURNING id;",
                    &[],
                )
                .await
                .unwrap();
            Ok(handler(
                Link::nothing(),
                Link::new(
                    result[0].get::<_, i64>(0).try_into().unwrap(),
                    T::ZERO,
                    T::ZERO,
                ),
            ))
        })
    }

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Flow {
        self.runtime.block_on(async {
            let any = self.constants.any;
            if query.is_empty() {
                let result = self
                    .transaction
                    .query("SELECT * FROM Links;", &[])
                    .await
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
                        .transaction
                        .query("SELECT * FROM Links WHERE id = $1;", &[&query[0].as_i64()])
                        .await
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
                let result = self.transaction.query(statement, &[]).await.unwrap();
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
        })
    }

    fn update_links(
        &mut self,
        query: &[T],
        change: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Error<T>> {
        self.runtime.block_on(async {
            let id = query[0];
            let source = change[1];
            let target = change[2];
            let old_links = self
                .transaction
                .query("SELECT * FROM Links WHERE id = $1;", &[&id.as_i64()])
                .await
                .unwrap();
            let (old_source, old_target) = (
                old_links[0].get::<_, i64>(1).try_into().unwrap(),
                old_links[0].get::<_, i64>(2).try_into().unwrap(),
            );
            self.transaction
                .query(
                    "UPDATE Links SET from_id = $1, to_id = $2 WHERE id = $3;",
                    &[&source.as_i64(), &target.as_i64(), &id.as_i64()],
                )
                .await
                .unwrap();
            Ok(handler(
                Link::new(id, old_source, old_target),
                Link::new(id, source, target),
            ))
        })
    }

    fn delete_links(&mut self, query: &[T], handler: WriteHandler<T>) -> Result<Flow, Error<T>> {
        self.runtime.block_on(async {
            let id = query[0];
            let result = self
                .transaction
                .query(
                    "DELETE FROM Links WHERE id = $1 RETURNING from_id, to_id",
                    &[&id.as_i64()],
                )
                .await
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
        })
    }
}

impl<T: LinkType> Doublets<T> for Transaction<'_, T> {
    fn get_link(&self, index: T) -> Option<Link<T>> {
        self.runtime.block_on(async {
            let result = self
                .transaction
                .query("SELECT * FROM Links WHERE id = $1", &[&index.as_i64()])
                .await;
            match result {
                Ok(rows) => {
                    let ref row = rows[0];
                    Some(Link::new(
                        row.get::<_, i64>(0).try_into().unwrap(),
                        row.get::<_, i64>(1).try_into().unwrap(),
                        row.get::<_, i64>(2).try_into().unwrap(),
                    ))
                }
                Err(_) => None,
            }
        })
    }
}
