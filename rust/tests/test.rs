use doublets::{unit, Doublets};
use linkspsql::Client;
use platform_mem::FileMappedMem;

#[test]
fn test() {
    let file = std::fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .open("db.links")
        .unwrap();
    let storage = FileMappedMem::new(file).unwrap();
    let mut links = unit::Store::<usize, _>::new(storage).unwrap();
    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }
    links.delete_all().unwrap();
}
