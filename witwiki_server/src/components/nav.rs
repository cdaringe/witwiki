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
  <div class="pl-4 pr-4">
    <input class='search w-full' placeholder="Search..." />
  </div>
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

#[derive(Eq, Hash, PartialEq)]
pub enum NavOption {
    Authenticated,
    EditPost(usize),
    AddPost,
}

fn get_action_links(nav_states: Vec<NavOption>) -> Vec<Link> {
    let is_authenticated = nav_states.contains(&NavOption::Authenticated);
    let mut action_links = if is_authenticated {
        (vec![
            Some(link("/logout", "logout")),
            Some(link("/x/user/settings", "user settings")),
        ])
        .into_iter()
        .filter_map(|it| it)
        .collect::<Vec<Link>>()
    } else {
        vec![link("/login", "login")]
    };
    for nav_opt in nav_states {
        match nav_opt {
            NavOption::EditPost(post_id) => {
                action_links.push(link(&format!("/wiki/edit/{}", post_id), "edit"));
            }
            NavOption::AddPost => {
                action_links.push(link("/wiki/create", "new post"));
            }
            NavOption::Authenticated => {}
        }
    }
    action_links
}

pub fn header(nav_links: &Vec<Link>, nav_states: Vec<NavOption>, children: &str) -> String {
    HeaderT {
        nav_links,
        action_links: &get_action_links(nav_states),
        children,
    }
    .render()
    .unwrap()
}
