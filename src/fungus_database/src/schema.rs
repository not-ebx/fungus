// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        world_id -> Int2,
        storage_mesos -> Int8,
        character_slots -> Int2,
        created_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        birthday -> Date,
        gender -> Int2,
        nx_cash -> Int4,
        maple_points -> Int4,
        vote_points -> Int4,
        account_type -> Int2,
        pic -> Nullable<Int2>,
        spw -> Nullable<Varchar>,
        ban_expire_date -> Nullable<Timestamp>,
        ban_reason -> Nullable<Text>,
        last_login -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    users,
);
