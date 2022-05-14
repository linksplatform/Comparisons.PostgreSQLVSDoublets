extern crate tokio;
extern crate criterion;

use crate::linkspsql;
use criterion::*;
use linkspsql::*;

async fn conn() -> LinksPSQL {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=mitron57 dbname=mitron57 password='mitron57' port=5432",
        NoTls
    ).await.unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    tokio::spawn(async move {
        if let Err(_) = connection.await {
            eprintln!("Connection error");
        }
    });
    LinksPSQL::new(client).expect("Can't borrow client")
}

fn bench(c: &mut Criterion) {
    let table = tokio::task::block_in_place(|| async {conn().await});
    c.bench_function("CreateMillionLinks", move |b| {
        b.iter_custom( |iters| async {
            let start = std::time::Instant::now();
            for i in 1..=iters {
                table.create(&[i;3]).await;
            }
            start.elapsed()
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);