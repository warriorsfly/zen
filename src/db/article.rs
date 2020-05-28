use crate::schema::articles;
use crate::schema::users;
use crate::{
    database::PoolType,
    errors::ServiceError,
    models::{
        article::{Article, ArticleJson},
        user::User,
    },
};
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    title: &'a str,
    description: &'a str,
    body: &'a str,
    author: i32,
    tags: &'a Vec<String>,
}

pub fn create(
    pool: &PoolType,
    author: i32,
    title: &str,
    description: &str,
    body: &str,
    tags: &Vec<String>,
) -> Result<ArticleJson, ServiceError> {
    let new_article = &NewArticle {
        title,
        description,
        body,
        author,
        tags,
    };
    let conn = pool
        .get()
        .map_err(|e| ServiceError::PoolError(e.to_string()))?;

    let author = users::table
        .find(author)
        .get_result::<User>(&conn)
        .expect("Error loading author");

    let art = diesel::insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(&conn)
        .expect("Error creating article")
        .attach(author, false);

    Ok(art)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::helpers::tests::get_pool;

    #[test]
    fn create_an_article() {
        let pool = get_pool();
        let res = create(
            &pool,
            1,
            "test create",
            "test create for unit",
            "东方红",
            &vec!["三".to_owned(), "四".to_owned()],
        );

        assert!(res.is_ok());
    }
}
