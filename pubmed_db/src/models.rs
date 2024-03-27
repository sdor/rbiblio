
use diesel::prelude::*;
use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = self::schema::chemicals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chemical {
    pub id: i32,
    pub pmid: i32,
    pub registry_number: String,
    pub name_of_substance: String,
    pub year: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::chemicals)]
pub struct NewChemical<'a> {
    pub pmid: i32,
    pub registry_number: &'a str,
    pub name_of_substance: &'a str,
    pub year: i32,
}

