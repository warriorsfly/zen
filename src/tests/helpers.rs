#[cfg(test)]
pub mod tests {

    use crate::{
        config::CONFIG,
        database::{init_pool, PoolType},
        state::{new_state, AppState},
    };
    use diesel::PgConnection;

    // Mock applicate state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
    // Mock applicate sql connection pool
    pub fn get_pool() -> PoolType {
        init_pool(CONFIG.clone()).unwrap()
    }
}
