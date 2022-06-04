use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use tokio_postgres::NoTls;

use linkspsql::LinksPSQL;

async fn connect() -> LinksPSQL {
    let (client, connection) = tokio_postgres::connect("", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(_) = connection.await {
            eprintln!("Connection error");
        }
    });
    LinksPSQL::new(client).await.unwrap()
}

fn create_million_links(c: &mut Criterion) {
    c.bench_function("create_million_links", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        b.to_async(&runtime).iter(|| async {
            let mut table = connect().await;
            for i in 1..=1_000 {
                table.create(&[i; 2]).await.unwrap();
            }
            table.complete().await.unwrap();
        });
        runtime.block_on(async {
            let mut table = connect().await;
            for i in 1..=1_000 {
                table.delete(&[i; 2]).await.unwrap();
            }
            table.complete().await.unwrap();
        });
    });
}

criterion_group!(benches, create_million_links);
criterion_main!(benches);
