table! {
    user_auth (id) {
        id -> Text,
        uid -> Text,
        identity_type -> Integer,
        identifier -> Text,
        certificate -> Text,
        login_session -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_base (id) {
        id -> Text,
        user_role -> Integer,
        register_source -> Integer,
        nick_name -> Text,
        gender -> Integer,
        birthday -> Nullable<Timestamp>,
        signature -> Text,
        mobile -> Text,
        mobile_bind_time -> Nullable<Timestamp>,
        email -> Text,
        email_bind_time -> Nullable<Timestamp>,
        avatar -> Text,
        avatar200 -> Text,
        avatar_source -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        push_token -> Text,
    }
}

table! {
    user_extra (uid) {
        uid -> Text,
        vendor -> Text,
        client_name -> Text,
        client_version -> Text,
        os_name -> Text,
        os_version -> Text,
        device_name -> Text,
        device_id -> Text,
        idfa -> Text,
        idfv -> Text,
        market -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        extend1 -> Text,
        extend2 -> Text,
        extend3 -> Text,
    }
}

table! {
    user_location (uid) {
        uid -> Text,
        curr_nation -> Text,
        curr_province -> Text,
        curr_city -> Text,
        curr_district -> Text,
        location -> Text,
        longitude -> Float,
        latitude -> Float,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    user_auth,
    user_base,
    user_extra,
    user_location,
);
