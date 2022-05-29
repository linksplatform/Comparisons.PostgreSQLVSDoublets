extern crate async_stream;
extern crate criterion;
extern crate tokio;

use std::process::Termination;

use async_stream::stream;
use criterion::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use crate::linkspsql::*;

async fn connect() -> LinksPSQL {
    let (client, connection) = tokio_postgres::connect("", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(_) = connection.await {
            eprintln!("Connection error");
        }
    });
    LinksPSQL::new(client).await.unwrap()
}

fn create_million_links(c: &mut Criterion) -> impl Termination {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let main = async move {
        let mut table = connect().await;
        c.bench_function("create_million_links", |b| {
            b.to_async(async_executor::FuturesExecutor)
                .iter(|| async move {
                    let stream = stream! {
                        for i in 0..1000000 {
                            table.create(&[i+1; 2]).await.unwrap();
                            yield;
                        }
                    };
                    pin_mut!(stream);
                    while let Some(_) = stream.next().await {}
                });
        });
    };
    runtime.block_on(main)
}

criterion_group!(benches, create_million_links);
criterion_main!(benches);
