use askama::Template;

#[derive(Template)]
#[template(
    source = r#"
<head>
  <style src="/css/tachyons.min.css"></style>
  <script src="/js/htmx.js"></script>
  <link rel='stylesheet' href='/css/water.css'>
  <link rel='stylesheet' href='/css/page.css'>
  <link rel='stylesheet' href='/css/utility.css'>
  <title>witwiki - {{ title }} </title>
  {{children}}
</head>
"#,
    ext = "html",
    escape = "none"
)]
struct HeadT<'a> {
    title: &'a str,
    children: &'a str,
}

pub fn head(title: &str, children_: Option<&str>) -> String {
    let children = children_.unwrap_or("");
    HeadT { title, children }.render().unwrap()
}
