extern crate tokio_postgres;
extern crate platform_data;
extern crate platform_num;

use std::borrow::Borrow;
use std::ops::{AddAssign, Deref};
use platform_num::LinkType;
use platform_data::LinksConstants;
use tokio::runtime::Runtime;

pub use tokio_postgres::{Client, NoTls, tls::NoTlsStream, Error, Row};
use tokio_postgres::types::FromSql;

pub struct LinksPSQL {
    pub index: u32,
    pub client: Client,
}
impl LinksPSQL
{
    pub fn new(client: Client) -> Result<Self, Error> {
        let run = Runtime::new().expect("Something went wrong with tokio::runtime");
        let index = run.block_on(
            client.query("SELECT * FROM Links", &[])
        ).expect("");
        Ok(Self { index, client })
    }

    pub async fn create(&mut self, substitution: &[T]) -> Result<Vec<Row>, Error> {
        self.index += T::one();
        self.client.query(
            &format!("INSERT INTO Links VALUES ({}, {}, {});",
                     self.index, substitution[0], substitution[1]),
            &[]
        ).await
    }

    pub async fn count(&self, restriction: &[T])  -> Result<i64, Error>
    {
        let mut result: Vec<Row> = Vec::new();
        let constants = LinksConstants::<T>::new();
        if restriction.eq(&[constants.any; 3]) {
            result = self.client.query("SELECT COUNT (*) FROM Links;", &[]).await?;
        }
        else if (&restriction[1..=2]).eq(&[constants.any;2]) && restriction[0] != constants.any {
            result = self.client.query(
                &format!("SELECT COUNT(*) FROM Links WHERE id = {};", restriction[0]),
                &[]
            ).await?;
        }
        else if (&restriction[1..=2]).ne(&[constants.any;2]) && restriction[0] == constants.any {
            result = self.client.query(
                &format!(
                    "SELECT COUNT (*) FROM Links WHERE from_id = {} AND to_id = {};",
                    restriction[1], restriction[2]
                ),
                &[]
            ).await?;
        }
        else if (&restriction[..=1]).eq(&[constants.any;2]) && restriction[2] != constants.any {
            result = self.client.query(
                &format!("SELECT COUNT (*) FROM Links WHERE to_id = {};", restriction[2]),
                &[]
            ).await?;
        }
        else if (&restriction[0], &restriction[2]) == (&constants.any, &constants.any) &&
            restriction[1] != constants.any {
            result = self.client.query(
                &format!("SELECT COUNT (*) FROM Links WHERE from_id = {};", restriction[1]),
                &[]
            ).await?;
        }
        Ok(result[0].get(0))
    }

    pub async fn update (
        &self,
        restriction: &[T],
        substitution: &[T]
    ) -> Result<Vec<Row>, Error> {
        if restriction.len() == 1 || restriction.len() == 3 {
            self.client.query(
                &format!("UPDATE Links SET from_id = {}, to_id = {} WHERE id = {};",
                         substitution[1], substitution[2], restriction[0]),
                &[]
            ).await
        }
        else {
            self.client.query(
                &format!(
                    "UPDATE Links SET from_id = {}, to_id = {} WHERE from_id = {} AND to_id = {};",
                    substitution[1], substitution[2], restriction[0], restriction[1]
                ),
                &[]
            ).await
        }
    }

    pub async fn delete(&self, restriction: &[u32]) -> Result<Vec<Row>, Error> {
        self.client.query(
            &format!("DELETE FROM Links WHERE id = {}", restriction[0]),
            &[]).await
    }

    pub async fn each(&self, restriction: &[T]) -> Result<Vec<Row>, Error>
    {
        let mut result: Vec<Row> = Vec::new();
        let constants = LinksConstants::<T>::new();
        if restriction.eq(&[constants.any; 3]) {
            result = self.client.query("SELECT * FROM Links;", &[]).await?;
        }
        else if (&restriction[1..=2]).eq(&[constants.any;2]) && restriction[0] != constants.any {
            result = self.client.query(
                &format!("SELECT * FROM Links WHERE id = {};", restriction[0]),
                &[]
            ).await?;
        }
        else if (&restriction[1..=2]).ne(&[constants.any;2]) && restriction[0] == constants.any {
            result = self.client.query(
                &format!(
                    "SELECT * FROM Links WHERE from_id = {} AND to_id = {};",
                    restriction[1], restriction[2]
                ),
                &[]
            ).await?;
        }
        else if (&restriction[..=1]).eq(&[constants.any;2]) && restriction[2] != constants.any {
            result = self.client.query(
                &format!("SELECT * FROM Links WHERE to_id = {};", restriction[2]),
                &[]
            ).await?;
        }
        else if (&restriction[0], &restriction[2]) == (&constants.any, &constants.any) &&
            restriction[1] != constants.any {
            result = self.client.query(
                &format!("SELECT * FROM Links WHERE from_id = {};", restriction[1]),
                &[]
            ).await?;
        }
        Ok(result)
    }
}