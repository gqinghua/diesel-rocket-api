table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    sys_role (id) {
        id -> Int4,
        name -> Varchar,
        del -> Varchar,
        create_date -> Varchar,
        parent_id -> Varchar,
    }
}

table! {
    sys_user (id) {
        id -> Int4,
        account -> Varchar,
        password -> Varchar,
        name ->Varchar,
        del -> Varchar,
    }
}

table! {
    sys_user_role (id) {
        id -> Int4,
        user_id -> Varchar,
        role_id -> Varchar,
        create_date -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    sys_role,
    sys_user,
    sys_user_role,
);
