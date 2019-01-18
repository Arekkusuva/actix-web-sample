table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::types::*;

    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}
