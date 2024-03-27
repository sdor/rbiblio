pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use models::Chemical;



pub fn insert_chemical(conn: &mut PgConnection, pmid: i32, registry_number: &str, name_of_substance: &str, year: i32) -> Result<usize, DieselError> {
    
    use self::schema::chemicals;
    use self::models::NewChemical;

    let new_chemical = NewChemical {
        pmid,
        registry_number,
        name_of_substance,
        year
    };
    diesel::insert_into(chemicals::table).values(&new_chemical).execute(&mut *conn)

}

pub fn find_by_pmid_and_name_of_substance(conn: &mut PgConnection, article_pmid: i32, name: &str)-> Result<Vec<i32>, DieselError> {
    use self::schema::chemicals::dsl::*;
    chemicals.filter(pmid.eq(&article_pmid)).filter(name_of_substance.eq(name)).select(id).load::<i32>(&mut *conn)
}

pub fn find_chemicals(conn: &mut PgConnection, retstart: i64, retmax: i64) -> Result<Vec<Chemical>,DieselError> {
    use self::schema::chemicals::dsl::*;
    chemicals.offset(retstart).limit(retmax).select(Chemical::as_select()).load(&mut *conn)
}

pub fn chemical_count_by_name_of_substance(conn: &mut PgConnection, name: &str) -> Result<i64,DieselError> {
    use self::schema::chemicals::dsl::*;
    chemicals.filter(name_of_substance.eq(name)).count().get_result::<i64>(&mut *conn)
}

pub fn find_chemical_by_name_of_substance(conn: &mut PgConnection, name: &str) -> Result<Vec<Chemical>,DieselError> {
    use self::schema::chemicals::dsl::*;
    chemicals.filter(name_of_substance.eq(name)).select(Chemical::as_select()).load(&mut *conn)
}

pub fn search_for_chemicals(conn: &mut PgConnection, search_term: &str, retstart: i64, retmax: i64) -> Result<Vec<Chemical>,DieselError> {
    use self::schema::chemicals::dsl::*;
    chemicals.filter(name_of_substance.ilike(search_term)).offset(retstart).limit(retmax).select(models::Chemical::as_select()).load(&mut *conn)
}

pub fn get_total_chemicals(conn: &mut PgConnection) -> Result<i64,DieselError> {
    use self::schema::chemicals::dsl::*;
    chemicals.count().get_result::<i64>(&mut *conn)
}
