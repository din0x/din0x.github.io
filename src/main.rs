use crate::template::link;
use gen_html::{DOCTYPE, Render, html};
use rust_website_gen::{App, Route};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

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
        div ."px-6 w-full flex flex-col items-center" {
            div ."mt-6 w-full max-w-240" {
                div ."flex gap-6" {
                    div ."max-md:hidden size-64 aspect-square bg-gray-900" {}
                    // img ."size-64" src: "/assets/me.png";
                    div ."font-mono" {
                        h1 ."mb-8 text-2xl md:text-3xl" {
                            span ."text-red-400" { "# " }
                            "Hi, I'm Robert"
                        }
                        p ."my-4 text-xl md:text-2xl text-mist-400" {
                            "I'm a software dev based in "
                            strong ."text-mist-300 font-normal" { "Kraków, Poland" }
                            ". Currently a student at ZSEL 1 high school in Kraków."
                        }
                        p ."text-xl md:text-2xl" {
                            "robertpoznanski.dev@gmail.com"
                            br;
                            (link("github.com/din0x", "https://github.com/din0x"))
                        }
                    }
                }
                div ."group/options" {
                    div ."pt-8 pb-4 md:py-6 flex gap-6 font-mono font-700 text-xl md:text-2xl" {
                        for (i, option) in options.iter().enumerate() {
                            label
                                ."cursor-pointer \
                                decoration-2 decoration-red-400 \
                                has-checked:bg-red-400 \
                                hover:has-not-checked:underline \
                                duration-100"
                            {
                                input ."hidden"
                                    r#type: "radio"
                                    name: "nav"
                                    value: (option)
                                    checked: (i == 0);

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
        div ."p-2 mb-2 rounded-lg border-2 text-mist-400 border-mist-800 bg-mist-900" {
            p ."mb-6 font-mono text-xl" {
                "Currently a student at "
                (link("ZSEL 1 high school", "https://zsel1.pl"))
                " in Kraków."
            }
        }
    }
}

fn strong(s: &str) -> impl Render {
    html! {
        strong ."text-mist-300 font-medium" {
            (s)
        }
    }
}

fn projects() -> impl Render {
    html! {
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

        let plotrs = Project {
            image: Some("/assets/projects/graphing.png"),
            code: Code::Open("https://github.com/din0x/plotrs"),
            description: &html! {
                (strong("Graphing calculator "))
                " supporting both 2D and 3D functions \
                    and equations written in Rust, uses a custom renderer built from scratch."
            },
            tags: vec![rust, wgpu, winit],
        };

        let renderer = Project {
            image: Some("/assets/projects/renderer.png"),
            code: Code::Open("https://github.com/din0x/plotrs"),
            description: &html! { (strong("2D/3D renderer")) " built on top of wgpu." },
            tags: vec![rust, wgpu],
        };

        let gen_html = Project {
            image: None,
            code: Code::Open("https://github.com/din0x/gen-html"),
            description: &html! {
                (strong("HTML templating library")) " for Rust. Made \
                for learning rust's macro system, used in my personal website."
            },
            tags: vec![rust, syn, quote],
        };

        let webdev_portfolio = Project {
            image: None,
            code: Code::Closed,
            description: &"Website development",
            tags: vec![astro, tailwind],
        };

        let lorem_impsum = Project {
            image: None,
            code: Code::Closed,
            description: &"Another project lorem impsum this is a long description omg",
            tags: vec![c, avr8, fusion],
        };

        let projects = [plotrs, gen_html, lorem_impsum, renderer, webdev_portfolio];

        div ."md:columns-2 gap-2" {
            for project in &projects {
                div
                    ."p-2 mb-2 \
                    rounded-lg border-2 border-mist-800 \
                    bg-mist-900 text-mist-400 \
                    break-inside-avoid"
                {
                    let Project { image, code, description, tags } = project;

                    if let Some(src) = image {
                        img ."mb-4" src: (src);
                    }

                    p ."mb-6 text-lg md:text-xl font-mono" { (description) }

                    div ."mb-2 flex gap-2" {
                        for tag in tags {
                            (tag)
                        }
                    }

                    match code {
                        Code::Open(repo) => a
                            ."inline-block mb-0  \
                            rounded-sm border-2 border-mist-400 \
                            pb-px px-2 \
                            text-md font-mono font-medium \
                            hover:bg-mist-800 hover:-translate-y-px \
                            duration-100"
                            href: (repo)
                            target: "_blank"
                        {
                            "View source code"
                        },
                        Code::Closed => {}
                    }
                }
            }
        }
    }
}

struct Project<'a> {
    image: Option<&'a str>,
    code: Code,
    description: &'a (dyn Render + 'a),
    tags: Vec<Tag>,
}

enum Code {
    Open(&'static str),
    Closed,
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
                meta charset: "UTF-8";
                meta name: "viewport" content: "width=device-width, initial-scale=1.0";
                title { (title) }
                link rel: "stylesheet" href: "/assets/css.css" ;
                script src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
            }
            body ."bg-mist-950 text-mist-300" {
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
