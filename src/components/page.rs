use askama::Template;

use crate::middleware::app_state::RequestState;

use super::nav;

#[derive(Template)]
#[template(
    source = r#"
<!DOCTYPE html>
<html>
  {{header}}
  <body>
    {{nav}}
    {{children}}
  <body>
</html>
"#,
    ext = "html",
    escape = "none"
)]
struct IndexT<'a> {
    nav: &'a str,
    header: &'a str,
    children: &'a str,
}

fn get_nav(request_state: &RequestState) -> String {
    let is_authenticated = request_state.get_cookies().get("foo").is_some();
    nav::header(
        &vec![nav::link("/", "home"), nav::link("/browse", "browse")],
        is_authenticated,
        "",
    )
}

pub fn page(request_state: &RequestState, header: &str, children: &str) -> String {
    IndexT {
        header,
        nav: &get_nav(request_state),
        children,
    }
    .render()
    .unwrap()
}
