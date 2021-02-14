table! {
    articles (id) {
        id -> Int4,
        author_id -> Int4,
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
        article_id -> Int4,
        author_id -> Int4,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorite_articles (user_id, article_id) {
        user_id -> Int4,
        article_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    followers (user_id, follower_id) {
        user_id -> Int4,
        follower_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Varchar,
        password -> Text,
        bio -> Text,
        avatar -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (author_id));
joinable!(favorite_articles -> articles (article_id));
joinable!(favorite_articles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    favorite_articles,
    followers,
    users,
);
