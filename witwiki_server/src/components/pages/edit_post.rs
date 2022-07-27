use askama::Template;

use crate::post::Post;

#[derive(Template)]
#[template(
    source = r#"
<main class='pa-4'>
  <form>
    <input type="text" value="{{title}}" class="w-full" />
    <div id="editor"></div>
    <pre id="editor-initial">{{body}}</pre>
    <script src="/js/vs/loader.js"></script>
    <script src="/js/init-editor.js"></script>
    <div class='pt-3' style='display: flex'>
      <input style='flex-grow: 1' name='change_description' placeholder='Describe edits authored' type='text' />
      <button style='flex-grow: 0' type='submit'>Submit</button>
    </div>
    <select id="theme-selector"></select>
    {{children}}
  </form>
</main>
"#,
    ext = "html",
    escape = "none"
)]
struct EditT<'a> {
    title: &'a str,
    body: &'a str,
    children: &'a str,
}

pub fn edit_post(post: Post, children: &str) -> String {
    EditT {
        title: &post.title,
        body: &post.body,
        children,
    }
    .render()
    .unwrap()
}
