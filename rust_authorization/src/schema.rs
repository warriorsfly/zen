table! {
    user_auth (id) {
        id -> Int4,
        uid -> Uuid,
        identity_type -> Int4,
        identifier -> Varchar,
        certificate -> Varchar,
        login_session -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_base (uid) {
        uid -> Uuid,
        user_role -> Int4,
        register_source -> Int4,
        user_name -> Varchar,
        nick_name -> Varchar,
        gender -> Int4,
        birthday -> Timestamp,
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
        uid -> Uuid,
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
        uid -> Uuid,
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

allow_tables_to_appear_in_same_query!(
    user_auth,
    user_base,
    user_extra,
    user_location,
);
