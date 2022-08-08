use async_trait::async_trait;
use platform_data::LinksConstants;
use std::cell::RefCell;
use tokio_postgres::{Error, GenericClient, Row};

type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait Cruds<'a, Executor>
where
    Executor: Send + GenericClient + Sync,
{
    fn zero(&mut self) -> &mut u64;

    fn one(&mut self) -> &Executor;

    async fn create(&mut self, substitution: &[u64]) -> Result<Vec<Row>> {
        let index = RefCell::new(self.zero());
        **index.borrow_mut() += 1;
        let index = **index.borrow();
        self.one()
            .query(
                &format!(
                    "INSERT INTO Links VALUES ({}, {}, {});",
                    index, substitution[0], substitution[1]
                ),
                &[],
            )
            .await
    }

    async fn each(&mut self, restriction: &[u64]) -> Result<Vec<Row>> {
        let mut result = Vec::new();
        let any = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self.one().query("SELECT * FROM Links;", &[]).await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .one()
                    .query(
                        &format!("SELECT * FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .one()
                    .query(
                        &format!("SELECT * FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .one()
                    .query(&format!("SELECT * FROM Links WHERE id = {};", id), &[])
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .one()
                    .query(
                        &format!(
                            "SELECT * FROM Links WHERE from_id = {} AND to_id = {};",
                            source, target
                        ),
                        &[],
                    )
                    .await?;
            }
            [] | [_, ..] => panic!("Constraints violation, use 'any'"),
        }
        Ok(result)
    }

    async fn count(&mut self, restriction: &[u64]) -> Result<i64> {
        let mut result = Vec::new();
        let any = LinksConstants::new().any;
        match restriction[..] {
            [any_id, any_source, any_target]
                if any_id == any && any_id == any_source && any_source == any_target =>
            {
                result = self
                    .one()
                    .query("SELECT COUNT (*) FROM Links;", &[])
                    .await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .one()
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .one()
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .one()
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE id = {}", id),
                        &[],
                    )
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .one()
                    .query(
                        &format!(
                            "SELECT COUNT (*) FROM Links WHERE from_id = {} AND to_id = {};",
                            source, target
                        ),
                        &[],
                    )
                    .await?;
            }
            [] | [_, ..] => panic!("Constraints violation, use 'any'"),
        }
        Ok(result[0].get(0))
    }

    async fn update(&mut self, restriction: &[u64], substitution: &[u64]) -> Result<Vec<Row>> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.one()
                .query(
                    &format!(
                        "UPDATE Links SET from_id = {}, to_id = {} WHERE id = {};",
                        substitution[0], substitution[1], restriction[0]
                    ),
                    &[],
                )
                .await
        } else {
            self.one()
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

    async fn delete(&mut self, restriction: &[u64]) -> Result<Vec<Row>> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.one()
                .query(
                    &format!("DELETE FROM Links WHERE id = {};", restriction[0]),
                    &[],
                )
                .await
        } else {
            self.one()
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
}
