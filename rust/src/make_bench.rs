#[macro_export]
macro_rules! make_bench {
    ($c:expr, $id:literal, $client:expr, $runtime:expr) => {
        $c.bench_function($id, |b| {
            b.iter(|| {
                $runtime.block_on(async {
                    for i in 1..=1000 {
                        $client.create(&[i; 2]).await.unwrap();
                    }
                });
            });
            $runtime.block_on(async {
                for i in 1..=1000 {
                    $client.delete(&[i; 2]).await.unwrap();
                }
            });
        });
    };
    ($c:expr, $id:literal, $client:expr, $runtime:expr, transaction) => {
        $c.bench_function($id, |b| {
            b.iter(|| {
                $runtime.block_on(async {
                    let mut transaction = $client.transaction().await.unwrap();
                    for i in 1..=1000 {
                        transaction.create(&[i; 2]).await.unwrap();
                    }
                    transaction.commit().await.unwrap();
                });
            });
            $runtime.block_on(async {
                let mut transaction = $client.transaction().await.unwrap();
                for i in 1..=1000 {
                    transaction.delete(&[i; 2]).await.unwrap();
                }
                transaction.commit().await.unwrap();
            });
        });
    };
}
