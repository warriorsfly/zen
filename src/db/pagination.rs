use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
const DEFAULT_PAGE_SIZE: i64 = 20;
pub trait Paginate: Sized {
    fn paginate(self, page_index: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page_index: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            page_index,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page_index: i64,
    page_size: i64,
}

impl<T> Paginated<T> {
    pub fn page_sizing(self, page_size: i64) -> Self {
        Paginated { page_size, ..self }
    }

    pub fn load_and_count_pages<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let page_size = self.page_size;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_page = (total as f64 / page_size as f64).ceil() as i64;
        Ok((records, total_page))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.page_size)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page_index - 1) * self.page_size;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
