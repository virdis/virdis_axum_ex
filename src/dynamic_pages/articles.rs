use askama::Template;

pub struct Article {
    pub title: String,
    pub body: String,
    pub date: String,
    pub published: bool,
}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate {
        title: String, 
    posts: Vec<Article>,
}

