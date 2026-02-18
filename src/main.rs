use gen_html::{DOCTYPE, Render, html};
use rust_website_gen::{App, Route};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use crate::template::link;

mod template;

fn main() {
    App::new()
        .route("/", root())
        .route("/assets", ServeDir("assets".into()))
        .build("target/html")
        .unwrap()
}

fn root() -> String {
    let options = ["projects", "education"];

    let content = html! {
        div ."flex flex-col items-center" {
            div ."w-full max-w-240" {
                div ."size-6" {}
                div ."flex" {
                    div ."size-64 mr-6 aspect-square bg-blue-950" {}
                    div ."font-mono" {
                        h1 ."text-2xl font-700" {
                            span ."text-red-400" { ">" } " whoami"
                        }
                        br;
                        p ."text-2xl" {
                            strong ."text-2xl" { "Work in progress " }
                            "hi, I'm Robert, software dev based in Poland. Currently a student at ZSEL 1 high school in Kraków."
                        }
                        br;
                        p ."text-2xl" {
                            "robertpoznanski.dev@gmail.com"
                            br;
                            (link("github.com/din0x", "https://github.com/din0x"))
                        }
                    }
                }
                div ."group/options" {
                    div ."py-6 flex gap-3 font-mono font-700 text-2xl select-none" {
                        for (i, option) in options.iter().enumerate() {
                            if i != 0 {
                                span { "/" }
                            }
                            label ."cursor-pointer has-[:checked]:text-red-400" {
                                match i {
                                    0 => input ."hidden" r#type: "radio" name: "selected" value: (option) checked;
                                    _ => input ."hidden" r#type: "radio" name: "selected" value: (option);
                                }
                                (option)
                            }
                        }
                    }
                    div ."hidden group-has-[input[value=projects]:checked]/options:block" {
                        ((projects()))
                    }
                    div ."hidden group-has-[input[value=education]:checked]/options:block" {
                        ((education()))
                    }
                }
                div ."size-64" {}
            }
        }
    };

    layout("Robert Poznański", content)
}

fn education() -> impl Render {
    html! {
        div ."p-2 mb-2 rounded-lg border-2 border-gray-800 bg-gray-900" {
            p ."mb-6 font-mono text-xl" {
                "Currently a student at "
                (link("ZSEL 1 high school", "https://zsel1.pl"))
                " in Kraków."
            }
        }
    }
}

fn projects() -> impl Render {
    let rust = Tag::new("rust", "#f7a87e");
    let syn = Tag::new("syn", "#cd516c");
    let quote = Tag::new("quote", "#9761ca");
    let wgpu = Tag::new("wgpu", "#0089eb");
    let winit = Tag::new("winit", "#e0b944");
    let astro = Tag::new("astro", "#e3399a");
    let tailwind = Tag::new("tailwind", "#74d4ff");
    let c = Tag::new("c", "#3996e3");
    let avr8 = Tag::new("avr8", "#f35446");
    let fusion = Tag::new("fusion", "#f47c31");

    let calculator = Project::new(
        "Graphing calculator supporting both 2D and 3D functions \
                and equations written in Rust, uses a custom renderer built from scratch.",
    )
    .image("/assets/projects/graphing.png")
    .tags([rust, wgpu, winit]);

    let renderer = Project::new("2D/3D renderer built on top of wgpu.")
        .image("/assets/projects/renderer.png")
        .tags([rust, wgpu]);

    let gen_html = Project::new(
        "HTML templating library for Rust. Made \
                for learning rust's macro system, used in my personal website.",
    )
    .tags([rust, syn, quote]);

    let webdev_portfolio = Project::new("Website development").tags([astro, tailwind]);

    let projects = [
        calculator,
        gen_html,
        Project::new("Another project lerem impsum this is a long description omg")
            .tags([c, avr8, fusion]),
        renderer,
        webdev_portfolio,
    ];

    html! {
        div ."columns-2 gap-2" {
            for project in &projects {
                div ."p-2 mb-2 rounded-lg border-2 border-gray-800 bg-gray-900 break-inside-avoid" {
                    if let Some(src) = project.img {
                        img ."mb-4" src: (src);
                    }
                    p ."mb-6 text-xl font-mono" { (project.desc) }
                    div ."mb-2 flex gap-2" {
                        for tag in &project.tags {
                            (tag)
                        }
                    }
                }
            }
        }
    }
}

struct Project {
    desc: &'static str,
    img: Option<&'static str>,
    tags: Vec<Tag>,
}

impl Project {
    fn new(desc: &'static str) -> Self {
        Self {
            desc,
            img: None,
            tags: Vec::new(),
        }
    }

    fn tags(mut self, iter: impl IntoIterator<Item = Tag>) -> Self {
        self.tags.extend(iter);
        self
    }

    fn image(mut self, url: &'static str) -> Self {
        self.img = Some(url);
        self
    }
}

#[derive(Clone, Copy)]
struct Tag {
    name: &'static str,
    color: &'static str,
}

impl Tag {
    fn new(name: &'static str, color: &'static str) -> Self {
        Self { name, color }
    }
}

impl Render for Tag {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = self.color;

        html! {
            span .(format!("rounded-sm pb-px border-2 border-[{color}] px-2 font-mono font-medium text-[{color}]")) {
                (self.name)
            }
        }
        .render_to(f)
    }
}

fn layout(title: &str, content: impl Render) -> String {
    html! {
        (DOCTYPE)
        html ."scheme-only-dark" {
            head {
                title { (title) }
                link rel: "stylesheet" href: "/assets/css.css" ;
                script src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
            }
            body ."bg-gray-950 text-gray-300" {
                // (nav())
                (content)
            }
        }
    }
    .to_string()
}

struct ServeDir(PathBuf);

impl Route for ServeDir {
    fn build(&self, path: &std::path::Path) -> io::Result<()> {
        walk_dir(&self.0, &mut |entry_path| {
            let dest = path.join(entry_path.strip_prefix(&self.0).unwrap());
            _ = fs::create_dir_all(dest.parent().unwrap());
            _ = fs::copy(entry_path, dest);
        })
    }
}

fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&Path)) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dir(&path, cb)?;
        } else {
            cb(&path)
        }
    }

    Ok(())
}
