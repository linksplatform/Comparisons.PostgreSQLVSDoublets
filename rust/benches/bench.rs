use std::time::Duration;

use async_stream::stream;
use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use linkspsql::LinksPSQL;
use tokio_postgres::NoTls;

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
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let mut table = connect().await;
                let client = connect().await.client;
                let stream = stream! {
                    for i in 0..10_000 {
                        yield table.create(&[i+1; 2]).await.unwrap();
                    }
                };
                pin_mut!(stream);
                while let Some(operation) = stream.next().await {
                    client.execute(&operation, &[]).await.unwrap();
                }
            });
    });
}

criterion_group!(benches, create_million_links);
criterion_main!(benches);
