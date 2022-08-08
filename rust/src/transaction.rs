use crate::cruds::Cruds;
use async_trait::async_trait;
use postgres::Error;
use tokio_postgres as postgres;

pub struct Transaction<'a>(u64, postgres::Transaction<'a>);

impl<'a> Transaction<'a> {
    pub fn new(transaction: postgres::Transaction<'a>, index: u64) -> Transaction<'a> {
        Transaction(index, transaction)
    }

    pub async fn commit(self) -> Result<(), Error> {
        self.1.commit().await
    }
}

#[async_trait]
impl<'a> Cruds<'a, postgres::Transaction<'a>> for Transaction<'a> {
    #[inline]
    fn zero(&mut self) -> &mut u64 {
        &mut self.0
    }

    #[inline]
    fn one(&mut self) -> &postgres::Transaction<'a> {
        &self.1
    }
}
