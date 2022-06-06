use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Template)]
#[template(
    source = r#"
<main class='pa-4'>
  <h2>{{title}}</h2>
  {{children}}
</main>
"#,
    ext = "html",
    escape = "none"
)]
struct PostT<'a> {
    title: &'a str,
    children: &'a str,
}

pub fn post(title: &str, children: &str) -> String {
    PostT {
        title,
        children: &markdown_to_html(children, &ComrakOptions::default()),
    }
    .render()
    .unwrap()
}
