table! {
    accounts (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

table! {
    articles (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        author -> Int4,
        tags -> Array<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        favorites_count -> Int4,
    }
}

table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        article -> Int4,
        author -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorites (user, article) {
        user -> Int4,
        article -> Int4,
    }
}

table! {
    follows (follower, followed) {
        follower -> Int4,
        followed -> Int4,
    }
}

table! {
    user_auth (id) {
        id -> Varchar,
        uid -> Varchar,
        identity_type -> Int4,
        identifier -> Varchar,
        certificate -> Varchar,
        login_session -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_base (id) {
        id -> Varchar,
        user_role -> Int4,
        register_source -> Int4,
        nick_name -> Varchar,
        gender -> Int4,
        birthday -> Nullable<Timestamp>,
        signature -> Varchar,
        mobile -> Varchar,
        mobile_bind_time -> Nullable<Timestamp>,
        email -> Varchar,
        email_bind_time -> Nullable<Timestamp>,
        avatar -> Varchar,
        avatar200 -> Varchar,
        avatar_source -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        push_token -> Varchar,
    }
}

table! {
    user_extra (uid) {
        uid -> Varchar,
        vendor -> Varchar,
        client_name -> Varchar,
        client_version -> Varchar,
        os_name -> Varchar,
        os_version -> Varchar,
        device_name -> Varchar,
        device_id -> Varchar,
        idfa -> Varchar,
        idfv -> Varchar,
        market -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        extend1 -> Varchar,
        extend2 -> Varchar,
        extend3 -> Varchar,
    }
}

table! {
    user_location (uid) {
        uid -> Varchar,
        curr_nation -> Varchar,
        curr_province -> Varchar,
        curr_city -> Varchar,
        curr_district -> Varchar,
        location -> Varchar,
        longitude -> Float8,
        latitude -> Float8,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(articles -> users (author));
joinable!(comments -> articles (article));
joinable!(comments -> users (author));
joinable!(favorites -> articles (article));
joinable!(favorites -> users (user));

allow_tables_to_appear_in_same_query!(
    accounts,
    articles,
    comments,
    favorites,
    follows,
    user_auth,
    user_base,
    user_extra,
    user_location,
    users,
);
