use surrealdb::sql::parse;
use surrealdb::{Datastore, Error, Session};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    //let ds = Datastore::new("memory").await?;
    let ds = Datastore::new("file://test.surreal.db").await?;
    let ses = Session::for_kv().with_ns("carltonj200").with_db("test");
    //let ast = parse("use ns carltonj2000 db db1; create person set name='carlton';")?;
    let ast = parse("create person set name='carlton', when=time::now();")?;
    let res = ds.process(ast, &ses, None, false).await?;
    println!("{:?}", res);

    let ast = parse("select * from person;")?;
    let res = ds.process(ast, &ses, None, false).await?;
    println!("{:?}", res);

    Ok(())
}
