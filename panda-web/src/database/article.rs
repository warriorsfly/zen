// use super::{pagination, Paginate};
use crate::schema::{articles, favorite_articles, followers, users};
use chrono::{DateTime, Utc};
use diesel::{self, prelude::*};
use panda_database::ConnectionPool;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

const SUFFIX_LEN: usize = 6;

#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub favorites_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    pub author_id: &'a i32,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub tag_list: &'a Vec<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "articles"]
pub struct ArticleChange {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "favorite_articles"]
pub struct NewFavoriteArticle {
    pub user_id: i32,
    pub article_id: i32,
}

/// 创建文章
pub fn create_article<'a>(
    conn: &'a ConnectionPool,
    author: &'a i32,
    title: &'a str,
    description: &'a str,
    body: &'a str,
    tags: &'a Vec<String>,
) -> QueryResult<Article> {
    let new_article = NewArticle {
        author_id: author,
        slug: &slugify(title),
        title,
        description,
        body,
        tag_list: tags,
    };
    diesel::insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(conn)
}

fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    } else {
        slug::slugify(title)
    }
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect()
}

#[derive(Deserialize, Default)]
pub struct ArticleFindData {
    tag: Option<String>,
    author: Option<String>,
    // favorited: Option<String>,
    offset: Option<i64>,
}

/// 查找文章
// pub fn search_articles<'a>(
//     conn: &'a ConnectionPool,
//     uid: Option<i32>,
//     params: &'a ArticleFindData,
// ) -> QueryResult<Vec<Article>> {
//     let mut query = articles::table
//         .inner_join(users::table)
//         .left_join(
//             favorite_articles::table.on(
//                 articles::id
//                     .eq(favorite_articles::article_id)
//                     .and(favorite_articles::user_id.eq(uid.unwrap())), // TODO: refactor
//             ),
//         )
//         .select((
//             articles::all_columns,
//             users::all_columns,
//             favorite_articles::user_id.nullable().is_not_null(),
//         ))
//         .into_boxed();

//     if let Some(ref author) = params.author {
//         query = query.filter(users::username.eq(author));
//     }

//     if let Some(ref tag) = params.tag {
//         query = query.or_filter(articles::tag_list.contapanda(vec![tag]));
//     }

//     query
//         .paginate(params.offset.unwrap_or_default())
//         // .offset(params.offset.unwrap_or_default())
//         // .limit(params.limit.unwrap_or(c))
//         .load_and_count_pages::<(Article, User, bool)>(conn)
//         .map(|(res, count)| {
//             (
//                 res.into_iter()
//                     .map(|(article, author, favorited)| article.attach(author, favorited))
//                     .collect::<Vec<Article>>(),
//                 count,
//             )
//         })
//         .map_err(|err| ServError::DataBaseError(err.to_string()))
// }

pub fn find_one_article<'a>(
    conn: &'a ConnectionPool,
    slug: &'a str,
    uid: &'a i32,
) -> QueryResult<Article> {
    articles::table
        .filter(articles::slug.eq(slug))
        .first::<Article>(conn)
}

#[derive(Deserialize, Default)]
pub struct FeedArticleData {
    limit: Option<i64>,
    offset: Option<i64>,
}

// pub fn feed_article<'a>(
//     conn: &'a ConnectionPool,
//     params: FeedArticleData,
//     uid: &i32,
// ) -> QueryResult<Vec<Article>> {
//     let arts = articles::table
//         .filter(
//             articles::author_id.eq_any(
//                 followers::table
//                     .select(followers::user_id)
//                     .filter(followers::follower_id.eq(uid)),
//             ),
//         )
//         .inner_join(users::table)
//         .left_join(
//             favorite_articles::table.on(articles::id
//                 .eq(favorite_articles::article_id)
//                 .and(favorite_articles::user_id.eq(uid))),
//         )
//         .select((
//             articles::all_columns,
//             users::all_columns,
//             favorite_articles::user_id.nullable().is_not_null(),
//         ))
//         .limit(params.limit.unwrap_or(pagination::DEFAULT_PAGE_SIZE))
//         .offset(params.offset.unwrap_or(0))
//         .load::<(Article, User, bool)>(conn)
//         .expect("Cannot load feed")
//         .into_iter()
//         .map(|(article, author, favorited)| article.attach(author, favorited))
//         .collect();
//     Ok(arts)
// }

// pub fn favorite_article<'a>(
//     conn: &'a ConnectionPool,
//     slug: &'a str,
//     uid: &'a i32,
// ) -> QueryResult<Article> {
//     conn.transaction::<_, diesel::result::Error, _>(|| {
//         let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
//             .set(articles::favorites_count.eq(articles::favorites_count + 1))
//             .get_result::<Article>(conn)?;
//         diesel::insert_into(favorite_articles::table)
//             .values((
//                 favorite_articles::user_id.eq(uid),
//                 favorite_articles::article_id.eq(article.id),
//             ))
//             .execute(conn)
//     })
// }

// pub fn unfavorite_article<'a>(
//     conn: &'a ConnectionPool,
//     slug: &'a str,
//     uid: &'a i32,
// ) -> QueryResult<u32> {
//     conn.transaction::<_, diesel::result::Error, _>(|| {
//         let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
//             .set(articles::favorites_count.eq(articles::favorites_count - 1))
//             .get_result::<Article>(conn)?;

//         diesel::delete(favorite_articles::table.find((uid, article.id))).execute(conn)
//     })
// }

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "articles"]
pub struct UpdateArticleData {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
    #[serde(skip)]
    slug: Option<String>,
    tag_list: Vec<String>,
}

pub fn update_article<'a>(
    conn: &'a ConnectionPool,
    slug: &'a str,
    uid: &'a i32,
    mut data: UpdateArticleData,
) -> QueryResult<Article> {
    if let Some(ref title) = data.title {
        data.slug = Some(slugify(&title));
    }
    diesel::update(articles::table.filter(articles::slug.eq(slug)))
        .set(&data)
        .get_result(conn)
}

pub fn delete_article<'a>(
    conn: &'a ConnectionPool,
    slug: &'a str,
    uid: &'a i32,
) -> QueryResult<usize> {
    diesel::delete(articles::table.filter(articles::slug.eq(slug).and(articles::author_id.eq(uid))))
        .execute(conn)
    //
}

pub fn get_tags<'a>(conn: &'a ConnectionPool) -> QueryResult<Vec<String>> {
    articles::table
        .select(diesel::dsl::sql("distinct unnest(tag_list)"))
        .load::<String>(conn)
}

fn is_favorite_article<'a>(conn: &'a ConnectionPool, article: &'a Article, uid: &'a i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(favorite_articles::table.find((uid, article.id))))
        .get_result(conn)
        .unwrap_or(false)
}
