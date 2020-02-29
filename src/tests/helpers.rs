#[cfg(test)]
pub mod tests {

    use crate::state::{new_state, AppState};

    // Mock applicate state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
