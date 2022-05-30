use askama::Template;

use crate::components::page::page;

#[derive(Template)]
#[template(
    source = r#"
<main class='pa-4'>
  {{title}}
  <h2>witwiki</h2>
  <p>
    This wiki is ready to do the best work.
  </p>
  {{children}}
</main>
"#,
    ext = "html",
    escape = "none"
)]
struct HomeT<'a> {
    title: &'a str,
    children: &'a str,
}

pub fn home(title: &str, children: &str) -> String {
    HomeT { title, children }.render().unwrap()
}
