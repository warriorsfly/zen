table! {
    user_auth (id) {
        id -> Int4,
        uid -> Int4,
        identity_type -> Int4,
        identifier -> Text,
        certificate -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_base (id) {
        id -> Int4,
        user_role -> Int4,
        register_source -> Int4,
        nick_name -> Text,
        gender -> Int4,
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
    }
}

table! {
    user_extra (id) {
        id -> Int4,
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
    user_location (id) {
        id -> Int4,
        curr_nation -> Text,
        curr_province -> Text,
        curr_city -> Text,
        curr_district -> Text,
        location -> Text,
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
