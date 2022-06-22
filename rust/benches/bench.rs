use criterion::{criterion_group, criterion_main, Criterion};
use tokio_postgres::NoTls;

use linkspsql::Client;

async fn connect() -> Client {
    let (client, connection) = tokio_postgres::connect("", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });
    Client::new(client).await.unwrap()
}

fn create_million_links(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("create_million_links", |b| {
        b.to_async(&runtime).iter(|| async {
            let mut client = connect().await;
            let mut transaction = client.get_transaction().await.unwrap();
            for i in 1..=1_000_000 {
                transaction.create(&[i; 2]).await.unwrap();
            }
            transaction.commit().await.unwrap();
        });
        runtime.block_on(async {
            let mut client = connect().await;
            let transaction = client.get_transaction().await.unwrap();
            for i in 1..=1_000_000 {
                transaction.delete(&[i; 2]).await.unwrap();
            }
            transaction.commit().await.unwrap();
        });
    });
}

criterion_group!(benches, create_million_links);
criterion_main!(benches);
