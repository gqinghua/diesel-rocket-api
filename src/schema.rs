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
        account -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        del -> Nullable<Int4>,
        create_date -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        is_active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    sys_user,
    users,
);
