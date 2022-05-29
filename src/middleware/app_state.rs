use axum::body::Body;
use axum::http::Request;
use by_address::ByAddress;
use cookie::Cookie;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum GlobalState {
    Nil,
}

#[derive(Debug, Clone)]
pub struct RequestState {
    pub cookies_by_name: HashMap<String, Cookie<'static>>,
}
pub type Req = Request<Body>;

#[derive(Debug)]
pub struct State{
    global: GlobalState,
    by_request: HashMap<ByAddress<Req>, RequestState>,
}

impl State {
    pub fn new() -> Self {
        State {
            global: GlobalState::Nil,
            by_request: HashMap::new(),
        }
    }

    pub fn get_request_state(self, req: &Req) -> &RequestState {
      self.by_request
            .get(&ByAddress(req))
            .expect("request not found")
    }

    pub fn set_request_state(self, req: &Req, rstate: RequestState) {
        self.by_request
            .borrow_mut()
            .insert(ByAddress(req), rstate)
            .expect("state not added");
    }
}
