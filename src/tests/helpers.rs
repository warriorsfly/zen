#[cfg(test)]
pub mod tests {

    use crate::{
        config::CONFIG,
        database::{init_pool, Pool},
        state::{new_state, AppState},
    };
    use diesel::SqliteConnection;

    // Mock applicate state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }

    pub fn get_pool() -> Pool<SqliteConnection> {
        init_pool::<SqliteConnection>(CONFIG.clone()).unwrap()
    }
}
