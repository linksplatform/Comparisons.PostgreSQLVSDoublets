extern crate tokio;
mod linkspsql;
//mod bench;
use linkspsql::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
    let mut links = LinksPSQL::<u32>::new(client).expect("Can't borrow client");
    let d: u32 = links.create(&[3u32;3]).await?[0].get(0);
    println!("{}", d);
    Ok(())
}
