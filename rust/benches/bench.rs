use criterion::{criterion_group, criterion_main, Criterion};
use tokio_postgres::{Error, NoTls};

use linkspsql::Client;

async fn connect() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect("", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            return Err(err);
        }
        Ok(())
    });
    Client::new(client).await
}

fn create_thousand_links_without_transaction(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut client = runtime.block_on(connect()).unwrap();
    c.bench_function("create_thousand_links_with_transaction", |b| {
        b.iter(|| {
            runtime.block_on(async {
                for i in 1..=1_000 {
                    client.create(&[i; 2]).await.unwrap();
                }
            });
        });
        runtime.block_on(async {
            for i in 1..=1_000 {
                client.delete(&[i; 2]).await.unwrap();
            }
        });
    });
}

fn create_thousand_links_with_transaction(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut client = runtime.block_on(connect()).unwrap();
    c.bench_function("create_thousand_links_with_transaction", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let mut transaction = client.transaction().await.unwrap();
                for i in 1..=1_000 {
                    transaction.create(&[i; 2]).await.unwrap();
                }
                transaction.commit().await.unwrap();
            });
        });
        runtime.block_on(async {
            let transaction = client.transaction().await.unwrap();
            for i in 1..=1_000 {
                transaction.delete(&[i; 2]).await.unwrap();
            }
            transaction.commit().await.unwrap();
        });
    });
}

criterion_group!(
    benches,
    create_thousand_links_without_transaction,
    create_thousand_links_with_transaction
);
criterion_main!(benches);
