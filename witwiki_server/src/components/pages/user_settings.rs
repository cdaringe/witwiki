#![allow(dead_code, unused)]

use askama::Template;

#[derive(Template)]
#[template(
    source = r#"
<main class='pa-4'>
  {{title}}
</main>
"#,
    ext = "html",
    escape = "none"
)]
struct EditT<'a> {
    title: &'a str,
    children: &'a str,
}

pub fn user_settings(title: &str, children: &str) -> String {
    EditT { title, children }.render().unwrap()
}
