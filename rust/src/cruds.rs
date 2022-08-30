use async_trait::async_trait;
use platform_data::LinksConstants;
use tokio_postgres::{Error, GenericClient, Row};

type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait Cruds<'a, Executor>
where
    Executor: Send + GenericClient + Sync,
{
    fn index(&mut self) -> &mut u64;

    fn executor(&mut self) -> &Executor;

    async fn create(&mut self, substitution: &[u64]) -> Result<Vec<Row>> {
        *self.index() += 1;
        let index = *self.index();
        self.executor()
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
                result = self.executor().query("SELECT * FROM Links;", &[]).await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .executor()
                    .query(
                        &format!("SELECT * FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .executor()
                    .query(
                        &format!("SELECT * FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .executor()
                    .query(&format!("SELECT * FROM Links WHERE id = {};", id), &[])
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .executor()
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
                    .executor()
                    .query("SELECT COUNT (*) FROM Links;", &[])
                    .await?;
            }
            [any_id, source, any_target] if any_id == any && any_id == any_target => {
                result = self
                    .executor()
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE from_id = {};", source),
                        &[],
                    )
                    .await?;
            }
            [any_id, any_source, target] if any_id == any && any_id == any_source => {
                result = self
                    .executor()
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE to_id = {};", target),
                        &[],
                    )
                    .await?;
            }
            [id, any_source, any_target] if any_source == any && any_source == any_target => {
                result = self
                    .executor()
                    .query(
                        &format!("SELECT COUNT (*) FROM Links WHERE id = {}", id),
                        &[],
                    )
                    .await?;
            }
            [any_id, source, target] if any_id == any => {
                result = self
                    .executor()
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
            self.executor()
                .query(
                    &format!(
                        "UPDATE Links SET from_id = {}, to_id = {} WHERE id = {};",
                        substitution[0], substitution[1], restriction[0]
                    ),
                    &[],
                )
                .await
        } else {
            self.executor()
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
            self.executor()
                .query(
                    &format!("DELETE FROM Links WHERE id = {};", restriction[0]),
                    &[],
                )
                .await
        } else {
            self.executor()
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
