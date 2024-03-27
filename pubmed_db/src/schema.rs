// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tsvector", schema = "pg_catalog"))]
    pub struct Tsvector;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tsvector;

    chemicals (id) {
        id -> Int4,
        pmid -> Int4,
        #[max_length = 255]
        registry_number -> Varchar,
        #[max_length = 255]
        name_of_substance -> Varchar,
        year -> Int4,
        name_of_substance_tsv -> Nullable<Tsvector>,
    }
}
