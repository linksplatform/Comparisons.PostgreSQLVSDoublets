use criterion::{criterion_group, criterion_main, Criterion};
use tokio_postgres::NoTls;

use linkspsql::LinksPSQL;

async fn connect() -> LinksPSQL {
    let (client, connection) = tokio_postgres::connect("", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });
    LinksPSQL::new(client).await.unwrap()
}

fn create_thousand_links(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("create_thousand_links", |b| {
        b.to_async(&runtime).iter(|| async {
            let mut table = connect().await;
            let mut statements = Vec::new();
            for i in 1..=1_000 {
                statements.push(table.create().await.unwrap());
            }
            for i in 0..1000 {
                table
                    .complete(&statements[i], &[(i + 1) as i64; 2])
                    .await
                    .unwrap();
            }
        });
        runtime.block_on(async {
            let mut table = connect().await;
            let mut statements = Vec::new();
            for i in 1..=1_000 {
                statements.push(table.delete(&[i; 2]).await.unwrap());
            }
            for i in 0..1000 {
                table.complete_without_args(&statements[i]).await.unwrap();
            }
        });
    });
}

criterion_group!(benches, create_thousand_links);
criterion_main!(benches);
