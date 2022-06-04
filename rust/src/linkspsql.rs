extern crate platform_data;
extern crate tokio_postgres;

pub use platform_data::LinksConstants;
pub use tokio_postgres::{tls::NoTlsStream, Client, Error, NoTls, Row, Statement};

pub struct LinksPSQL {
    pub index: u32,
    pub client: Client,
    statements: Vec<Statement>,
    query_id: u32,
}

impl LinksPSQL {
    pub async fn new(client: Client) -> Result<Self, Error> {
        let index = client.query("SELECT * FROM Links;", &[]).await?.len() as u32;
        Ok(Self {
            index,
            client,
            statements: Vec::new(),
            query_id: 0,
        })
    }

    pub async fn create(&mut self, substitution: &[u32]) -> Result<u32, Error> {
        self.index += 1;
        let statement = self
            .client
            .prepare(&format!(
                "INSERT INTO Links VALUES ({}, {}, {});",
                self.index, substitution[0], substitution[1]
            ))
            .await
            .unwrap();
        self.statements.push(statement);
        self.query_id += 1;
        Ok(self.query_id)
    }

    pub async fn count(&self, restriction: &[u32]) -> Result<i64, Error> {
        let mut result = Vec::new();
        let any: u32 = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self.client.query("SELECT COUNT(*) FROM Links;", &[]).await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .client
                    .query(
                        &format!("SELECT COUNT(*) FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .client
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .client
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE id = {};", id),
                        &[],
                    )
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .client
                    .query(
                        &format!(
                            "SELECT COUNT (*) FROM Links WHERE from_id = {} AND to_id = {};",
                            source, target
                        ),
                        &[],
                    )
                    .await?;
            }
            [] | [_, ..] => todo!(),
        }
        Ok(result[0].get(0))
    }

    pub async fn update(
        &self,
        restriction: &[u32],
        substitution: &[u32],
    ) -> Result<Vec<Row>, Error> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.client
                .query(
                    &format!(
                        "UPDATE Links SET from_id = {}, to_id = {} WHERE id = {};",
                        substitution[1], substitution[2], restriction[0]
                    ),
                    &[],
                )
                .await
        } else {
            self.client
                .query(
                    &format!(
                        "UPDATE Links SET from_id = {}, to_id = {} WHERE from_id = {} AND to_id = {};",
                        substitution[1], substitution[2], restriction[0], restriction[1]
                    ),
                    &[],
                )
                .await
        }
    }

    pub async fn delete(&mut self, restriction: &[u32]) -> Result<u32, Error> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.statements.push(
                self.client
                    .prepare(&format!("DELETE FROM Links WHERE id = {};", restriction[0]))
                    .await
                    .unwrap(),
            )
        } else {
            self.statements.push(
                self.client
                    .prepare(&format!(
                        "DELETE FROM Links WHERE from_id = {} AND to_id = {};",
                        restriction[0], restriction[1]
                    ))
                    .await
                    .unwrap(),
            )
        }
        self.query_id += 1;
        Ok(self.query_id)
    }

    pub async fn each(&self, restriction: &[u32]) -> Result<Vec<Row>, Error> {
        let mut result = Vec::new();
        let any: u32 = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self.client.query("SELECT * FROM Links;", &[]).await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .client
                    .query(
                        &format!("SELECT * FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .client
                    .query(
                        &format!("SELECT * FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .client
                    .query(&format!("SELECT * FROM Links WHERE id = {};", id), &[])
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .client
                    .query(
                        &format!(
                            "SELECT * FROM Links WHERE from_id = {} AND to_id = {};",
                            source, target
                        ),
                        &[],
                    )
                    .await?;
            }
            [] | [_, ..] => todo!(),
        }
        Ok(result)
    }

    pub async fn complete(&mut self) -> Result<(), Error> {
        for statement in &self.statements {
            self.client.execute(statement, &[]).await.unwrap();
        }
        self.statements.clear();
        self.query_id = 0;
        Ok(())
    }

    pub async fn complete_by_index(&mut self, index: u32) -> Result<(), Error> {
        self.client
            .execute(&self.statements[(index - 1) as usize], &[])
            .await
            .unwrap();
        self.statements.remove((index - 1) as usize);
        Ok(())
    }
}
