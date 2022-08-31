use crate::cruds::Cruds;
use async_trait::async_trait;
use postgres::Error;
use tokio_postgres as postgres;

pub struct Transaction<'a> {
    index: u64,
    transaction: postgres::Transaction<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(transaction: postgres::Transaction<'a>, index: u64) -> Transaction<'a> {
        Transaction { index, transaction }
    }

    pub async fn commit(self) -> Result<(), Error> {
        self.transaction.commit().await
    }
}

impl<'a> Cruds<'a, postgres::Transaction<'a>> for Transaction<'a> {
    #[inline]
    fn index(&mut self) -> &mut u64 {
        &mut self.index
    }

    #[inline]
    fn executor(&mut self) -> &postgres::Transaction<'a> {
        &self.transaction
    }
}
