use crate::cruds::Cruds;
use crate::Transaction;
use async_trait::async_trait;
use postgres::Error;
use tokio_postgres as postgres;

pub struct Client {
    index: u64,
    client: postgres::Client,
}

impl Client {
    pub async fn new(client: postgres::Client) -> Result<Client, Error> {
        let index = client.query("SELECT * FROM Links;", &[]).await?.len() as u64;
        Ok(Self { index, client })
    }

    pub async fn transaction(&mut self) -> Result<Transaction<'_>, Error> {
        let transaction = self.client.transaction().await.unwrap();
        Ok(Transaction::new(transaction, self.index))
    }
}

impl Cruds<'_, postgres::Client> for Client {
    #[inline]
    fn index(&mut self) -> &mut u64 {
        &mut self.index
    }

    #[inline]
    fn executor(&mut self) -> &postgres::Client {
        &self.client
    }
}
