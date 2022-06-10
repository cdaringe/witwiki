use crate::{
    components::{head, page, pages::post},
    middleware::app_state::RequestState,
    page_routes::wiki::common::get_page_post,
};
use axum::{extract::Path, response::Html, Extension};

pub async fn handle_get(
    Path(id): Path<usize>,
    request_state: Extension<RequestState>,
) -> Html<String> {
    let post = get_page_post(&request_state.db, id).await;
    Html(page::page(
        &request_state,
        &head::head(&post.title, None),
        vec![],
        &post::post(&post.title, &post.body),
    ))
}
