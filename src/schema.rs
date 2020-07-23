table! {
    articles (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        tag_list -> Array<Text>,
        favorites_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    comments (id) {
        id -> Int4,
        article_id -> Uuid,
        user_id -> Uuid,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorite_articles (user_id, article_id) {
        user_id -> Uuid,
        article_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    followers (user_id, follower_id) {
        user_id -> Uuid,
        follower_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password -> Text,
        bio -> Nullable<Text>,
        avatar -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(favorite_articles -> articles (article_id));
joinable!(favorite_articles -> users (user_id));

allow_tables_to_appear_in_same_query!(articles, comments, favorite_articles, followers, users,);
