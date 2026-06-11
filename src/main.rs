use gen_html::{Render, html};
use rust_website_gen::{App, Route};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::template::*;

mod blog;
mod markdown;
mod template;

fn main() {
    App::new()
        .route("/", root())
        .route("/experience", experience())
        .route("/blog", blog::app())
        .route("/assets", ServeDir("assets".into()))
        .build("target/html")
        .unwrap()
}

fn about_me() -> impl Render {
    html! {
        div ."font-mono text-xl md:text-2xl text-mist-400" {
            p ."mb-4" {
                "Hi there, I'm " (strong("Robert")) ", highschool student in "
                (strong("Kraków, Poland")) ". "
            }
            p ."mb-4" {
                "Currently working on a stratospheric balloon."
            }
            p ."mb-8" {
                "I love " (strong("math")) ", coding and working out."
            }
            p ."mb" {
                "Interested? Reach out"
            }
            p ."mb-4" {
                "via email "
                (strong("robertpoznanski.dev@gmail.com"))
                br;
                " or on " (link("github.com/din0x", "https://github.com/din0x"))
            }
        }
    }
}

fn root() -> String {
    let content = html! {
        div ."px-6 w-full flex flex-col items-center" {
            div ."w-full max-w-240" {
                div ."my-10" {
                    (about_me())
                }
                (projects())
            }
            div ."size-64" {}
        }
    };

    layout("Robert Poznański", "/", content)
}

fn experience() -> String {
    let arcan = frame(html! {
        p ."mb-6 font-mono text-xl" {
            "Internship at " (strong("Arcan Studios")) " in " (strong("Granada, Spain"))
            ". Learned 3D modeling, rigging and animating characters in " (strong("Blender")) ". "
            "Imported custom models and animations to an " (strong("Unreal Engine")) " game. "
            "Programmed character movement, level obstacles and game logic and using Blueprint"
            "visual scripting."
        }
        div ."flex gap-2" {
            (UNREAL_ENGINE)
            (BLENDER)
        }
    });

    let content = html! {
        div ."px-6 w-full flex flex-col items-center" {
            div ."mt-6 w-full max-w-240" {
                (arcan)
            }
            div ."size-64" {}
        }
    };

    layout("Robert Poznański", "/experience", content)
}

fn strong(r: impl Render) -> impl Render {
    html! {
        strong ."text-mist-300 font-medium" {
            (r)
        }
    }
}

fn projects() -> impl Render {
    html! {
        let plotrs = Project {
            image: Some("/assets/projects/graphing.png"),
            code: Code::Open("https://github.com/din0x/plotrs"),
            description: &html! {
                (strong("Graphing calculator "))
                " supporting both 2D and 3D functions \
                    and equations written in Rust, uses a custom renderer built from scratch."
            },
            tags: vec![RUST, WGPU, WINIT],
        };

        let renderer = Project {
            image: Some("/assets/projects/renderer.png"),
            code: Code::Open("https://github.com/din0x/plotrs"),
            description: &html! { (strong("2D/3D renderer")) " built on top of wgpu." },
            tags: vec![RUST, WGPU],
        };

        let gen_html = Project {
            image: None,
            code: Code::Open("https://github.com/din0x/gen-html"),
            description: &html! {
                (strong("HTML templating library")) " for Rust. Made \
                for learning rust's macro system, used in my personal website."
            },
            tags: vec![RUST, SYN, QUOTE],
        };

        let ubx = Project {
            image: None,
            code: Code::Open("https://github.com/din0x/ubx"),
            description: &html! {
                (strong("UBX protocol library")) " for Rust providing packet encoding and stream \
                decoding with automatic recovery and synchronization."
            },
            tags: vec![RUST, UBX],
        };

        let avr = Project {
            image: None,
            code: Code::Open("https://github.com/din0x/avr"),
            description: &html! {
                (strong("AVR HAL")) " library. Provides safe abstractions for accessing peripherals of ATmega MCUs."
            },
            tags: vec![RUST, AVR8, C],
        };

        let projects = [plotrs, gen_html, ubx, avr, renderer];

        div ."md:columns-2 gap-2" {
            for project in &projects {
                (frame(html! {

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
                        Code::_Closed => {}
                    }
                }))
            }
        }
    }
}

struct Project<'a> {
    image: Option<&'a str>,
    code: Code,
    description: &'a (dyn Render + 'a),
    tags: Vec<Badge>,
}

enum Code {
    Open(&'static str),
    _Closed,
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
