use platform_data::LinksConstants;
use postgres::{Error, Row};
use tokio_postgres as postgres;

pub struct Transaction<'a>(postgres::Transaction<'a>, u64);

impl<'a> Transaction<'a> {
    pub fn new(transaction: postgres::Transaction<'a>, index: u64) -> Transaction<'a> {
        Transaction(transaction, index)
    }

    pub async fn commit(self) -> Result<(), Error> {
        self.0.commit().await
    }

    pub async fn create(&mut self, substituiton: &[u64]) -> Result<Vec<Row>, Error> {
        self.1 += 1;
        self.0
            .query(
                &format!(
                    "INSERT INTO Links VALUES({}, {}, {});",
                    self.1, substituiton[0], substituiton[1]
                ),
                &[],
            )
            .await
    }

    pub async fn count(&self, restriction: &[u64]) -> Result<i64, Error> {
        let mut result = Vec::new();
        let any = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self.0.query("SELECT COUNT (*) FROM Links;", &[]).await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .0
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .0
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .0
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE id = {}", id),
                        &[],
                    )
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .0
                    .query(
                        &format!(
                            "SELECT COUNT (*) FROM Links WHERE from_id = {} AND to_id = {};",
                            source, target
                        ),
                        &[],
                    )
                    .await?;
            }
            [] | [_, ..] => eprintln!("Constraints violation, use \"any\" "),
        }
        Ok(result[0].get(0))
    }

    pub async fn update(
        &self,
        restriction: &[u64],
        substitution: &[u64],
    ) -> Result<Vec<Row>, Error> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.0
                .query(
                    &format!(
                        "UPDATE Links SET from_id = {}, to_id = {} WHERE id = {};",
                        substitution[0], substitution[1], restriction[0]
                    ),
                    &[],
                )
                .await
        } else {
            self.0
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

    pub async fn delete(&self, restriction: &[u64]) -> Result<Vec<Row>, Error> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.0
                .query(
                    &format!("DELETE FROM Links WHERE id = {};", restriction[0]),
                    &[],
                )
                .await
        } else {
            self.0
                .query(
                    &format!(
                        "DELETE FROM Links WHERE from_id = {} AND to_id = {};",
                        restriction[0], restriction[1]
                    ),
                    &[],
                )
                .await
        }
    }

    pub async fn each(&self, restriction: &[u64]) -> Result<Vec<Row>, Error> {
        let mut result = Vec::new();
        let any = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self.0.query("SELECT * FROM Links;", &[]).await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .0
                    .query(
                        &format!("SELECT * FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .0
                    .query(
                        &format!("SELECT * FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .0
                    .query(&format!("SELECT * FROM Links WHERE id = {};", id), &[])
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .0
                    .query(
                        &format!(
                            "SELECT * FROM Links WHERE from_id = {} AND to_id = {};",
                            source, target
                        ),
                        &[],
                    )
                    .await?;
            }
            [] | [_, ..] => eprintln!("Constraints violation, use \"any\""),
        }
        Ok(result)
    }
}
