extern crate pubmed;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use pubmed::ArticleChemical;
use pubmed_db::find_by_pmid_and_name_of_substance;
use tokio::runtime::Handle;
use std::env;
use tokio::{io, sync::mpsc};


fn article_chemical_exists(conn: &mut PgConnection, pmid: i32, name_of_substance: &str) -> bool {
    match find_by_pmid_and_name_of_substance(conn, pmid, name_of_substance) {
        Ok(found) => found.len() > 0,
        Err(_) => false,
    }
}

fn insert_chemical(
    pool: Pool<ConnectionManager<PgConnection>>,
    chemical: ArticleChemical,
) {
    let runtime_handle = Handle::current();
    runtime_handle.block_on(async {
        let mut conn = pool.get().unwrap();
        let pmid: i32 = chemical.pmid as i32;
        let registry_number = chemical.registry_number.as_str();
        let name_of_substance = chemical.name_of_substance.as_str();
        let year: i32 = chemical.year as i32;
        if !article_chemical_exists(&mut conn, pmid, name_of_substance) {
            let _ = pubmed_db::insert_chemical(
                &mut conn,
                pmid,
                registry_number,
                name_of_substance,
                year,
            );
        }
    })
}

#[tokio::main]
async fn main() -> io::Result<()> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pubmed_baseline = env::var("PUBMED_BASELINE").expect("PUBMED_BASELINE must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder().build(manager).unwrap();

    let (tx, mut rx) = mpsc::channel(10);
    let producer_handle = tokio::spawn(pubmed::directory_articles(pubmed_baseline, tx));
    while let Some(article) = rx.recv().await {
        let chemicals = article.chemicals();
        for chemical in chemicals {
            // println!("{:?}", chemical);
            let pool = pool.clone();
            let _ = tokio::task::spawn_blocking(|| {insert_chemical(pool, chemical)}).await;
        }
    }
    producer_handle.await.expect("Producer task panicked")
}
