use crate::template::*;
use gen_html::{Render, html};
use std::fmt;

pub fn root() -> String {
    let arcan = Experience {
        company: "Arcan Studios",
        role: "Game Developer Intern",
        timespan: "May 2026",
        location: "Granada, Spain",
        badges: vec![UNREAL_ENGINE, BLENDER],
    };

    let cavatina = Experience {
        company: "Cavatina",
        role: "Software Developer Intern",
        timespan: "Jul 2026",
        location: "Kraków, Poland",
        badges: vec![RUST, AXUM, JWT, REACT, TAILWIND, DOCKER, COOLIFY, PYTHON],
    };

    let jobs = [cavatina, arcan];

    let content = html! {
        div ."px-6 w-full flex flex-col items-center font-serif" {
            div ."w-full max-w-180" {
                div ."my-10" {
                    div ."text-xl md:text-2xl text-mist-400 tracking-wide" {
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

                let h1_class = "pl-2 border-l-4 border-red-400 text-2xl first-letter:text-[1.125em] mb-4 mt-10 font-semibold";

                h1 .(h1_class) {
                    "Experience"
                }

                for job in &jobs {
                    div ."mb-4" {
                        (job)
                    }
                }

                h1 .(h1_class) {
                    "Projects"
                }

                (projects())
            }
            div ."size-64" {}
        }
    };

    layout("Robert Poznański", "/", content)
}

struct Experience {
    company: &'static str,
    role: &'static str,
    location: &'static str,
    timespan: &'static str,
    badges: Vec<Badge>,
}

impl Render for Experience {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        html! {
            let Self { company, role, location, timespan, badges } = self;

            div ."pl-2 border-l-4 border-red-400 border-dotted" {
                div ."flex justify-between text-xl font-normal" {
                    span ."text-2xl" {
                        (company)
                    }

                    span ."" {
                        (location)
                    }
                }

                div ."mb-1 flex justify-between text-xl font-normal" {
                    span ."text-mist-400" {
                        (role)
                    }

                    span ."italic text-mist-400" {
                        (timespan)
                    }
                }

                div ."mt-4 flex gap-2" {
                    for badge in badges {
                        (badge)
                    }
                }
            }
        }
        .render_to(f)
    }
}

fn projects() -> impl Render {
    use crate::template::*;

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

        div ."gap-2" {
            for project in &projects {
                (project)
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

impl Render for Project<'_> {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        html! {
            let Project { image, code, description, tags } = self;

            div ."mb-4 border-l-4 border-red-400 border-dotted pl-2 flex flex-row gap-2 justify-between" {
                div ."flex flex-col gap-2 justify-between" {
                    p ."md:text-xl text-xl text-mist-400" {
                        (description)
                    }

                    div {
                        match code {
                            Code::Open(repo) => a
                                ."inline-block mb-2 \
                                rounded-sm border-2 border-slate-500 \
                                pb-px px-2 \
                                text-md text-slate-400 font-mono font-medium \
                                hover:bg-mist-800 hover:-translate-y-px \
                                duration-100"
                                href: (repo)
                                target: "_blank"
                            {
                                "Github"
                            },
                            Code::_Closed => {}
                        }

                        div ."flex gap-2" {
                            for tag in tags {
                                (tag)
                            }
                        }
                    }
                }

                // div ."" {

                // }

                if let Some(src) = image {
                    img ."w-65 h-fit aspect-16/9 object-cover rounded-md" src: (src);
                }
            }
        }.render_to(f)
    }
}

enum Code {
    Open(&'static str),
    _Closed,
}
