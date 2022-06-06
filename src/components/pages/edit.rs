use askama::Template;

#[derive(Template)]
#[template(
    source = r#"
<main class='pa-4'>
  {{title}}
  <select id="theme-selector"></select>
  <div id="editor"></div>
  <script src="/js/vs/loader.js"></script>
  <script src="/js/init-editor.js"></script>
  {{children}}
</main>
"#,
    ext = "html",
    escape = "none"
)]
struct EditT<'a> {
    title: &'a str,
    children: &'a str,
}

pub fn edit(title: &str, children: &str) -> String {
    EditT { title, children }.render().unwrap()
}
