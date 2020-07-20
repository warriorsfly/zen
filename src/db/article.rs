use super::Paginate;
use crate::{
    auth::PrivateClaim,
    database::DatabasePoolType,
    errors::ServiceError,
    models::{Article, ArticleJson, User},
    schema::{articles, favorite_articles, users},
};
use diesel::{self, insert_into, prelude::*};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use slug;
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
    pool: &DatabasePoolType,
    author: &Uuid,
    title: &str,
    description: &str,
    body: &str,
    tags: &Vec<String>,
) -> Result<ArticleJson, ServiceError> {
    let new_article = NewArticle {
        author_id: author,
        slug: &slugify(title),
        title,
        description,
        body,
        tag_list: tags,
    };
    let conn = pool.get()?;
    let author = users::table
        .find(author)
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

    let new_article = insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

    Ok(new_article.attach(author, 0, false))
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
pub fn search(
    pool: &DatabasePoolType,
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
        query = query.filter(diesel::dsl::sql(&format!(
            "articles.id IN(SELECT favorites.article_id FROM favorites WHERE favorites.user_id={}",
            result
        )));
    }

    query
        .paginate(params.offset.unwrap_or_default())
        // .offset(params.offset.unwrap_or_default())
        // .limit(params.limit.unwrap_or(c))
        .load_and_count_pages::<(Article, User, bool)>(&conn)
        .map(|(res, count)| {
            (
                res.into_iter()
                    .map(|(article, author, favorited)| article.attach(author, 10, favorited))
                    .collect::<Vec<ArticleJson>>(),
                count,
            )
        })
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}

// fn find_one(pool: &DatabasePoolType, slug: &str, uid: Uuid) -> Result<ArticleJson, ServiceError> {
//     let conn = pool.get()?;
//     let article = articles::table
//         .filter(articles::slug.eq(slug))
//         .first::<Article>(&conn)
//         .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

//     let article_json = article.attach(author, 10, favorited)
// }

// fn is_favorited(conn:&PgConnection,article:&Article,uid:&Uuid)->bool{
//     use diesel::dsl::exists;
//     use diesel::select;

//     select(exists(favorite_articles::table.find((uid,article.id))))
// }

// fn populate(conn:&PgConnection,article:Article,favorites_count:u32,favorited:bool)->ArticleJson{
//     let author = users::table.find(article.author_id).get_result::<User>(conn).expect("Error loading authors");

//     article.attach(author, favorites_count, favorited)

// }
