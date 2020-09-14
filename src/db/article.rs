use super::{pagination, Paginate};
use crate::{
    database::DatabaseConnectionPool,
    errors::ServiceError,
    models::{Article, ArticleJson, User},
    schema::{articles, favorite_articles, followers, users},
};
use diesel::{self, insert_into, prelude::*};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use uuid::Uuid;

const SUFFIX_LEN: usize = 6;
#[derive(Debug, Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    pub author_id: &'a Uuid,
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
    pub user_id: Uuid,
    pub article_id: Uuid,
}

/// 创建文章
pub fn create_article(
    pool: &DatabaseConnectionPool,
    author: &Uuid,
    title: &str,
    description: &str,
    body: &str,
    tags: &Vec<String>,
) -> Result<ArticleJson, ServiceError> {
    let conn = pool.get()?;
    let new_article = NewArticle {
        author_id: author,
        slug: &slugify(title),
        title,
        description,
        body,
        tag_list: tags,
    };
    let author = users::table
        .find(author)
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

    let new_article = insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

    Ok(new_article.attach(author, false))
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
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}

#[derive(Deserialize, Default)]
pub struct ArticleFindData {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    offset: Option<i64>,
}

/// 查找文章
pub fn search_articles(
    pool: &DatabaseConnectionPool,
    uid: Option<Uuid>,
    params: &ArticleFindData,
) -> Result<(Vec<ArticleJson>, i64), ServiceError> {
    let conn = pool.get()?;
    let mut query = articles::table
        .inner_join(users::table)
        .left_join(
            favorite_articles::table.on(
                articles::id
                    .eq(favorite_articles::article_id)
                    .and(favorite_articles::user_id.eq(uid.unwrap())), // TODO: refactor
            ),
        )
        .select((
            articles::all_columns,
            users::all_columns,
            favorite_articles::user_id.nullable().is_not_null(),
        ))
        .into_boxed();

    if let Some(ref author) = params.author {
        query = query.filter(users::username.eq(author));
    }

    if let Some(ref tag) = params.tag {
        query = query.or_filter(articles::tag_list.contains(vec![tag]));
    }
    if let Some(ref favorited) = params.favorited {
        let result = users::table
            .select(users::id)
            .filter(users::username.eq(favorited))
            .get_result::<Uuid>(&conn)?;

        // query = query.filter(
        //     favorite_articles::table::select(favorite_articles::article_id)
        //         .filter(favorite_articles::user_id.eq(result))
        //         .ex(articles::id),
        // );
        // query = query.filter(diesel::dsl::sql(&format!(
        //     "articles.id IN(SELECT favorites.article_id FROM favorites WHERE favorites.user_id={}",
        //     result
        // )).);
    }

    query
        .paginate(params.offset.unwrap_or_default())
        // .offset(params.offset.unwrap_or_default())
        // .limit(params.limit.unwrap_or(c))
        .load_and_count_pages::<(Article, User, bool)>(&conn)
        .map(|(res, count)| {
            (
                res.into_iter()
                    .map(|(article, author, favorited)| article.attach(author, favorited))
                    .collect::<Vec<ArticleJson>>(),
                count,
            )
        })
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}

pub fn find_one_article(
    pool: &DatabaseConnectionPool,
    slug: &str,
    uid: &Uuid,
) -> Result<ArticleJson, ServiceError> {
    let conn = pool.get()?;
    let article = articles::table
        .filter(articles::slug.eq(slug))
        .first::<Article>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;
    let favorited = is_favorite_article(&conn, &article, &uid);

    Ok(populate_article(&conn, article, favorited))
}

#[derive(Deserialize, Default)]
pub struct FeedArticleData {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn feed_article(
    pool: &DatabaseConnectionPool,
    params: FeedArticleData,
    uid: &Uuid,
) -> Result<Vec<ArticleJson>, ServiceError> {
    let conn = pool.get()?;
    let arts = articles::table
        .filter(
            articles::author_id.eq_any(
                followers::table
                    .select(followers::user_id)
                    .filter(followers::follower_id.eq(uid)),
            ),
        )
        .inner_join(users::table)
        .left_join(
            favorite_articles::table.on(articles::id
                .eq(favorite_articles::article_id)
                .and(favorite_articles::user_id.eq(uid))),
        )
        .select((
            articles::all_columns,
            users::all_columns,
            favorite_articles::user_id.nullable().is_not_null(),
        ))
        .limit(params.limit.unwrap_or(pagination::DEFAULT_PAGE_SIZE))
        .offset(params.offset.unwrap_or(0))
        .load::<(Article, User, bool)>(&conn)
        .expect("Cannot load feed")
        .into_iter()
        .map(|(article, author, favorited)| article.attach(author, favorited))
        .collect();
    Ok(arts)
}

pub fn favorite_article(
    pool: &DatabaseConnectionPool,
    slug: &str,
    uid: &Uuid,
) -> Result<ArticleJson, ServiceError> {
    let conn = pool.get()?;
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(articles::favorites_count.eq(articles::favorites_count + 1))
            .get_result::<Article>(&conn)?;
        diesel::insert_into(favorite_articles::table)
            .values((
                favorite_articles::user_id.eq(uid),
                favorite_articles::article_id.eq(article.id),
            ))
            .execute(&conn)?;

        Ok(populate_article(&conn, article, true))
    })
    .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}

pub fn unfavorite_article(
    pool: &DatabaseConnectionPool,
    slug: &str,
    uid: &Uuid,
) -> Result<ArticleJson, ServiceError> {
    let conn = pool.get()?;
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(articles::favorites_count.eq(articles::favorites_count - 1))
            .get_result::<Article>(&conn)?;

        diesel::delete(favorite_articles::table.find((uid, article.id))).execute(&conn)?;

        Ok(populate_article(&conn, article, false))
    })
    .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}

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

pub fn update_article(
    pool: &DatabaseConnectionPool,
    slug: &str,
    uid: &Uuid,
    mut data: UpdateArticleData,
) -> Result<ArticleJson, ServiceError> {
    let conn = pool.get()?;
    if let Some(ref title) = data.title {
        data.slug = Some(slugify(&title));
    }
    // TODO: check for not_found
    let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
        .set(&data)
        .get_result(&conn)?;

    let favorited = is_favorite_article(&conn, &article, uid);
    Ok(populate_article(&conn, article, favorited))
}

pub fn delete_article(
    pool: &DatabaseConnectionPool,
    slug: &str,
    uid: &Uuid,
) -> Result<usize, ServiceError> {
    let conn = pool.get()?;
    diesel::delete(articles::table.filter(articles::slug.eq(slug).and(articles::author_id.eq(uid))))
        .execute(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))
    //
}

pub fn get_tags(pool: &DatabaseConnectionPool) -> Result<Vec<String>, ServiceError> {
    let conn = pool.get()?;
    articles::table
        .select(diesel::dsl::sql("distinct unnest(tag_list)"))
        .load::<String>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}

fn is_favorite_article(conn: &PgConnection, article: &Article, uid: &Uuid) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(favorite_articles::table.find((uid, article.id))))
        .get_result(conn)
        .unwrap_or(false)
}

fn populate_article(conn: &PgConnection, article: Article, favorited: bool) -> ArticleJson {
    let author = users::table
        .find(article.author_id)
        .get_result::<User>(conn)
        .expect("Error loading authors");

    article.attach(author, favorited)
}
