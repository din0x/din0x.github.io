use crate::{
    markdown,
    template::{self, frame},
};
use gen_html::html;
use rust_website_gen::App;
use std::{fs, io};

pub fn app() -> App {
    let posts = read_all_posts().unwrap();

    let mut app = App::new();
    app.route("/", root(&posts));

    for p in &posts {
        app.route(format!("/{}", p.route), post(p));
    }

    app
}

fn post(post: &Post) -> String {
    let html = html! {
        div ."px-6 w-full flex flex-col items-center" {
            div ."mb-10 w-full max-w-180 text-mist-400 text-lg md:text-xl" {
                (markdown::render(&post.markdown))
            }
            div ."size-64" {}
        }
    };

    template::layout("Blog", &format!("/blog/{}", post.route), html)
}

fn root(posts: &[Post]) -> String {
    let html = html! {
        div ."px-6 w-full flex flex-col items-center" {
            div ."my-10 w-full max-w-180 font-mono text-mist-400 text-lg md:text-xl" {
                for post in posts {
                    (frame(html! {
                        a href: (post.route) {
                            (post.route)
                        }
                    }))
                }
            }
            div ."size-64" {}
        }
    };

    template::layout("Blog", "/blog", html)
}

fn read_all_posts() -> io::Result<Vec<Post>> {
    let mut vec = Vec::new();

    for entry in fs::read_dir("blog")? {
        let entry = entry?;
        let markdown = fs::read_to_string(entry.path())?;

        vec.push(Post {
            route: entry.path().file_prefix().unwrap().display().to_string(),
            markdown,
        });
    }

    Ok(vec)
}

struct Post {
    route: String,
    markdown: String,
}
