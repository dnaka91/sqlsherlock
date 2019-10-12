table! {
    sqlite_master(type_, name) {
        #[sql_name = "type"]
        type_ -> Text,
        name -> Text,
    }
}
