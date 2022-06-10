use askama::Template;

use crate::middleware::app_state::RequestState;

use super::nav::{self, NavOption};

#[derive(Template)]
#[template(
    source = r#"
<!DOCTYPE html>
<html>
  {{header}}
  <body>
    {{nav}}
    {{children}}
  </body>
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

fn get_nav(request_state: &RequestState, mut nav_states: Vec<NavOption>) -> String {
    let mut nav_states = vec![];
    let is_authenticated = request_state.get_cookies().get("foo").is_some();
    if is_authenticated {
        nav_states.push(nav::NavOption::Authenticated);
    }
    nav::header(
        &vec![nav::link("/", "home"), nav::link("/browse", "browse")],
        nav_states,
        "",
    )
}

pub fn page(
    request_state: &RequestState,
    header: &str,
    nav_states: Vec<NavOption>,
    children: &str,
) -> String {
    IndexT {
        header,
        nav: &get_nav(request_state, nav_states),
        children,
    }
    .render()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::fixtures::get_request_state;

    #[test]
    fn test_get_page() {
        let rendered = page(&get_request_state(), "<head>test_fake</head>", vec![], "");
        assert!(rendered.contains("<html>") && rendered.contains("</html>"));
        assert!(rendered.contains("<body>") && rendered.contains("</body>"));
        assert!(rendered.contains("<head>test_fake</head>"));
    }
}
