use crate::{
    database::ConnectionPool,
    errors::ServError,
    models::{Comment, CommentJson, User},
    schema::{articles, comments, users},
};

use diesel::{self, prelude::*};
use uuid::Uuid;
#[derive(Insertable)]
#[table_name = "comments"]
struct NewComment<'a> {
    body: &'a str,
    author_id: &'a Uuid,
    article_id: &'a Uuid,
}

pub fn create_comment(
    pool: &ConnectionPool,
    author: Uuid,
    slug: &str,
    body: &str,
) -> Result<CommentJson, ServError> {
    let conn = pool.get()?;
    let article_id = articles::table
        .select(articles::id)
        .filter(articles::slug.eq(slug))
        .get_result::<Uuid>(&conn)
        .map_err(|_| ServError::DataBaseError("Canot find the article".into()))?;
    let author = users::table
        .find(&author)
        .get_result::<User>(&conn)
        .map_err(|err| ServError::DataBaseError(err.to_string()))?;

    let comment = NewComment {
        body,
        author_id: &author.id,
        article_id: &article_id,
    };

    let comment = diesel::insert_into(comments::table)
        .values(comment)
        .get_result::<Comment>(&conn)?;

    Ok(comment.attach(author))
}

//TODO 后续需要修改,要考虑分页的情况
pub fn find_comments_by_slug(
    pool: &ConnectionPool,
    slug: &str,
) -> Result<Vec<CommentJson>, ServError> {
    let conn = pool.get()?;
    let result = comments::table
        .inner_join(articles::table)
        .inner_join(users::table)
        .select((comments::all_columns, users::all_columns))
        .filter(articles::slug.eq(slug))
        .get_results::<(Comment, User)>(&conn)
        .expect("Cannot load comments");

    let result = result
        .into_iter()
        .map(|(comment, author)| comment.attach(author))
        .collect();

    Ok(result)
}

pub fn delete_comment(
    pool: &ConnectionPool,
    author: Uuid,
    slug: &str,
    comment_id: &str,
) -> Result<(), ServError> {
    use diesel::dsl::exists;
    use diesel::select;
    let conn = pool.get()?;
    let belongs_to_author_result = select(exists(
        articles::table.filter(articles::slug.eq(slug).and(articles::author_id.eq(author))),
    ))
    .get_result::<bool>(&conn)
    .map_err(|err| ServError::DataBaseError(err.to_string()))?;
    if belongs_to_author_result {
        let _result = diesel::delete(comments::table.find(Uuid::parse_str(comment_id).unwrap()))
            .execute(&conn)
            .map_err(|err| ServError::DataBaseError(err.to_string()))?;
    }

    Ok(())
}
