use askama::Template;

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

pub fn page(header: &str, nav: &str, children: &str) -> String {
    IndexT {
        header,
        nav,
        children,
    }
    .render()
    .unwrap()
}
