
use std::time::Duration;

use async_stream::stream;
use criterion::{Criterion, criterion_group, criterion_main};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
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
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let mut table = connect().await;
                let stream = stream! {
                    for i in 0..1_000_000 {
                        table.create(&[i+1; 2]).await.unwrap();
                        yield;
                    }
                };
                pin_mut!(stream);
                while let Some(_) = stream.next().await {
                }
            });
    });
}

criterion_group!(benches, create_million_links);
criterion_main!(benches);
