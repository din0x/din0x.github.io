use gen_html::{DOCTYPE, Render, html};
use rust_website_gen::{App, Route};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

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
                            "hi, I'm Robert, software dev based in Poland. Currently in Zsel 1 highschool in Kraków."
                        }
                        br;
                        p ."text-2xl" {
                            "robertpoznanski.dev@gmail.com"
                            br;
                            a ."group relative" href: "https://github.com/din0x" {
                                span ."absolute bottom-0 -z-1 block w-full h-[2px] bg-red-400 duration-50
                                    group-hover:h-1" {}
                                "github.com/din0x"
                            }
                        }
                    }
                }
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
                ((projects()))
                div ."size-64" {}
            }
        }
    };

    layout("Robert Poznański", content)
}

fn projects() -> impl Render {
    let projects = [
        Project {
            desc: "Graphing calculator supporting both 2d and 3d functions and equations written in Rust.",
            // tags: vec![Tag::new("webgpu", "#0093ff"), Tag::new("winit", "#f0c751")],
            tags: vec![
                Tag::new("rust", "#f7a87e"),
                Tag::new("webgpu", "#0089eb"),
                Tag::new("winit", "#e0b944"),
            ],
            img: Some("/assets/projects/graphing.png"),
        },
        Project {
            desc: "Another project lerem impsum this is a long description omg",
            tags: vec![Tag::new("c", "#3996e3"), Tag::new("avr8", "#f35446"), Tag::new("fusion", "#f47c31")],
            img: None,
        },
        Project {
            desc: "Website development",
            tags: vec![Tag::new("astro", "#e3399a"), Tag::new("tailwind", "#74d4ff")],
            img: None,
        },
    ];

    html! {
        div ."columns-2 gap-2" {
            for project in &projects {
                div ."p-2 pb-6 mb-2 rounded-lg bg-gray-900 break-inside-avoid" {
                    if let Some(src) = project.img {
                        img ."mb-4" src: (src);
                        // div ."w-full aspect-square bg-blue-950" {}
                    }
                    p ."mb-6 text-xl font-mono" { (project.desc) }
                    div ."flex gap-2" {
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
        html {
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
