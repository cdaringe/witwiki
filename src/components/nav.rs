use askama::Template;

pub struct Link {
    href: String,
    children: String,
}

pub fn link(href: &str, children: &str) -> Link {
    Link {
        href: href.to_owned(),
        children: children.to_owned(),
    }
}
#[derive(Template)]
#[template(
    source = r#"
<nav>
  {% for link in links %}
  <a href={{link.href}}>{{link.children}}</a>
  {% endfor %}
</nav>
"#,
    ext = "html",
    escape = "none"
)]
struct NavT<'a> {
    links: &'a Vec<Link>,
    children: &'a str,
}

pub fn nav(links: &Vec<Link>, children: &str) -> String {
    NavT { links, children }.render().unwrap()
}
