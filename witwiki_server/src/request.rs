#[cfg(test)]
pub mod fixtures {
    use std::sync::Arc;

    use crate::{db, middleware::app_state::RequestState};
    pub fn get_request_state() -> RequestState {
        RequestState::new(Arc::new(db::Db::new_in_mem()))
    }
}
