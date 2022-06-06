use platform_data::LinksConstants;
use tokio_postgres::{Client, Error, Row, Statement};

pub struct LinksPSQL {
    index: u32,
    client: Client,
}

impl LinksPSQL {
    pub async fn new(client: Client) -> Result<Self, Error> {
        let index = client.query("SELECT * FROM Links;", &[]).await?.len() as u32;
        Ok(Self { index, client })
    }

    pub async fn create(&mut self) -> Result<Statement, Error> {
        self.index += 1;
        self.client
            .prepare(&format!(
                "INSERT INTO Links VALUES ({}, $1, $2);",
                self.index
            ))
            .await
    }

    pub async fn count(&self, restriction: &[u32]) -> Result<i64, Error> {
        let mut result = Vec::new();
        let any: u32 = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self
                    .client
                    .query("SELECT COUNT (*) FROM Links;", &[])
                    .await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .client
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE from_id = {};", source),
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
                        &format!("SELECT COUNT (*) FROM Links WHERE id = {}", id),
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
                        substitution[0], substitution[1], restriction[0]
                    ),
                    &[],
                )
                .await
        } else {
            self.client
                .query(
                    &format!("UPDATE Links SET from_id = {}, to_id = {} WHERE from_id = {} AND to_id = {};",
                        substitution[0],
                        substitution[1],
                        restriction[0],
                        restriction[1]),
                    &[],
                )
                .await
        }
    }

    pub async fn delete(&mut self, restriction: &[u32]) -> Result<Statement, Error> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.client
                .prepare(&format!("DELETE FROM Links WHERE id = {};", restriction[0]))
                .await
        } else {
            self.client
                .prepare(&format!(
                    "DELETE FROM Links WHERE from_id = {} AND to_id = {};",
                    restriction[0], restriction[1]
                ))
                .await
        }
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

    pub async fn complete(&self, statement: &Statement, args: &[i64]) -> Result<u64, Error> {
        self.client.execute(statement, &[&args[0], &args[1]]).await
    }

    pub async fn complete_without_args(&self, statement: &Statement) -> Result<u64, Error> {
        self.client.execute(statement, &[]).await
    }
}
