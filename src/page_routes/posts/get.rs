use crate::{
    components::{head, page, pages::post},
    middleware::app_state::RequestState,
    post::Post,
};
use axum::response::Html;

pub fn get(request_state: &RequestState, post: Post, id: usize) -> Html<String> {
    Html(page::page(
        request_state,
        &head::head(&post.title, None),
        &post::post(&post.title, &post.body),
    ))
}
