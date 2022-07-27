use crate::{
    components::{head, nav::NavOption, page, pages::post},
    middleware::app_state::RequestState,
};
use axum::{extract::Path, response::Html, Extension};

use super::common::get_page_post;

pub async fn handle_get(
    Path(id): Path<usize>,
    request_state: Extension<RequestState>,
) -> Html<String> {
    let post = get_page_post(&request_state.db, id).await;
    Html(page::page(
        &request_state,
        &head::head(&post.title, None),
        vec![NavOption::EditPost(id)],
        &post::post(&post.title, &post.body),
    ))
}
