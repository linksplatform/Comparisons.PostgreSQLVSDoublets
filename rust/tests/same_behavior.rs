use doublets::{Doublets, data::Flow, mem::Global, split, unit};
use linkspsql::{Benched, Client, Exclusive, Sql, Transaction};

fn rows(store: &impl Doublets<usize>, query: &[usize]) -> Vec<(usize, usize, usize)> {
    let mut rows = Vec::new();
    store.each_links(query, &mut |link| {
        rows.push((link.index, link.source, link.target));
        Flow::Continue
    });
    rows.sort_unstable();
    rows
}

fn exercise(store: &mut impl Doublets<usize>) {
    let any = store.constants().any;
    let first = store.create_point().unwrap();
    let second = store.create_point().unwrap();
    assert_eq!((first, second), (1, 2));
    assert_eq!(store.count(), 2);
    assert_eq!(rows(store, &[]), vec![(1, 1, 1), (2, 2, 2)]);
    assert_eq!(rows(store, &[first]), vec![(1, 1, 1)]);
    assert!(store.get_link(999).is_none());
    for query in [[any, second, second], [any, second, any], [any, any, second]] {
        assert_eq!(rows(store, &query), vec![(2, 2, 2)]);
        assert_eq!(store.count_by(query), 1);
    }
    store.update(second, 0, 0).unwrap();
    assert_eq!(rows(store, &[second, any, any]), vec![(2, 0, 0)]);
    store.update(second, second, second).unwrap();
    let third = store.create_point().unwrap();
    let fourth = store.create_point().unwrap();
    assert_eq!((third, fourth), (3, 4));
    for id in linkspsql::created_links(2, 2).rev() {
        store.delete(id).unwrap();
    }
    assert_eq!(rows(store, &[]), vec![(1, 1, 1), (2, 2, 2)]);
    store.delete(second).unwrap();
    assert_eq!(rows(store, &[]), vec![(1, 1, 1)]);
    store.delete(first).unwrap();
    assert_eq!(store.count(), 0);
}

fn lifecycle(store: &mut (impl Benched + Doublets<usize>)) {
    {
        let mut fork = store.fork();
        exercise(&mut *fork);
    }
    {
        let mut fork = store.fork();
        assert_eq!(fork.create_point().unwrap(), 1);
        assert_eq!(fork.create_point().unwrap(), 2);
    }
    let mut fork = store.fork();
    assert_eq!(fork.count(), 0);
    assert_eq!(fork.create_point().unwrap(), 1);
}

#[test]
fn doublets_same_behavior() {
    lifecycle(&mut unit::Store::<usize, Global<_>>::setup(()).unwrap());
    lifecycle(&mut split::Store::<usize, Global<_>, Global<_>>::setup(()).unwrap());
    let directory = std::env::temp_dir().join(format!("linkspsql-test-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let united = directory.join("united.links");
    lifecycle(
        &mut unit::Store::<usize, doublets::mem::FileMapped<_>>::setup(united.to_str().unwrap())
            .unwrap(),
    );
    let data = directory.join("data.links");
    let index = directory.join("index.links");
    lifecycle(&mut split::Store::<usize, doublets::mem::FileMapped<_>, doublets::mem::FileMapped<_>>::setup((data.to_str().unwrap(), index.to_str().unwrap())).unwrap());
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
#[ignore = "requires a disposable PostgreSQL database; set POSTGRES_CONNECTION"]
fn postgresql_same_behavior() {
    let mut client = Exclusive::<Client<usize>>::new(linkspsql::connect().unwrap());
    client.drop_table().unwrap();
    client.create_table().unwrap();
    lifecycle(&mut client);
    let mut connection = linkspsql::connect().unwrap();
    let mut transaction = Exclusive::<Transaction<'_, usize>>::setup(&mut connection).unwrap();
    lifecycle(&mut transaction);
}
