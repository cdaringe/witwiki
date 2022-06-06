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
<div class='header'>
  <nav class='pa-3'>
    <nav class='dib'>
      {% for link in nav_links %}
      <a href={{link.href}}>{{link.children}}</a>
      {% endfor %}
    </nav>
    <nav class='fr'>
      {% for link in action_links %}
      <a href={{link.href}}>{{link.children}}</a>
      {% endfor %}
    </nav>
  </nav>
  <input class='search mt-3 ml-3 mr-3' placeholder="Search..." />
  {{children}}
</div>
"#,
    ext = "html",
    escape = "none"
)]
struct HeaderT<'a> {
    action_links: &'a Vec<Link>,
    nav_links: &'a Vec<Link>,
    children: &'a str,
}

fn get_action_links(is_authenticated: bool) -> Vec<Link> {
    let auth_link = if is_authenticated {
        link("/logout", "logout")
    } else {
        link("/login", "login")
    };
    let mut action_links = if is_authenticated {
        vec![
            link("/x/user/settings", "user settings"),
            link("/edit", "edit"),
            link("/x/add", "add"),
        ]
    } else {
        vec![]
    };
    action_links.insert(0, auth_link);
    action_links
}

pub fn header(nav_links: &Vec<Link>, is_authenticated: bool, children: &str) -> String {
    HeaderT {
        nav_links,
        action_links: &get_action_links(is_authenticated),
        children,
    }
    .render()
    .unwrap()
}
