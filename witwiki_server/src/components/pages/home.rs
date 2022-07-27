use askama::Template;

use crate::post::Post;

#[derive(Template)]
#[template(
    source = r#"
<main class='pa-4'>
  {{title}}
  <h2>witwiki</h2>
  <p>
    This wiki is ready to do the best work.
  </p>
  {{recent_posts}}
  {{children}}
</main>
"#,
    ext = "html",
    escape = "none"
)]
struct HomeT<'a> {
    title: &'a str,
    recent_posts: &'a str,
    children: &'a str,
}

pub async fn home(title: &str, recent_posts: Vec<Post>, children: &str) -> String {
    HomeT {
        title,
        recent_posts: &format!(
            "<ul>{}</ul",
            recent_posts
                .into_iter()
                .map(|x| format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    format!("/wiki/{}", x.id),
                    x.title
                ))
                .collect::<Vec<String>>()
                .join("")
        ),
        children,
    }
    .render()
    .unwrap()
}
