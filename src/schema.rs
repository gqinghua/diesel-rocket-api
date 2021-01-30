table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    sys_user (id) {
        id -> Int4,
        account -> Varchar,
        password -> Varchar,
        name -> Varchar,
        del -> Int4,

    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    sys_user,
);
