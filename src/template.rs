use gen_html::{DOCTYPE, Render, html};
use std::fmt;

pub fn strong(r: impl Render) -> impl Render {
    html! {
        strong ."text-mist-300 font-medium" {
            (r)
        }
    }
}

pub fn link(r: impl Render, href: &str) -> impl Render {
    html! {
        a
            ."relative inline bg-gradient-to-t from-red-400 to-red-400 \
            bg-size-[100%_2px] bg-no-repeat bg-left-bottom text-mist-300 \
            hover:bg-size-[100%_100%] \
            duration-50"

            href: (href)
            target: "_blank"
        {
            (r)
        }
    }
}

pub fn frame(r: impl Render) -> impl Render {
    html! {
        div ."p-2 rounded-lg border-1 border-mist-800 text-mist-400 bg-mist-900 break-inside-avoid" {
            (r)
        }
    }
}

pub struct Badge(pub &'static str, pub &'static str);

impl Render for Badge {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = self.1;

        html! {
            span .(format!("inline-block rounded-sm pb-px border-2 border-[{color}] px-2 font-mono font-medium text-[{color}]")) {
                (self.0)
            }
        }.render_to(f)
    }
}

macro_rules! badges {
    ( $( $Ident:ident => $color:literal ),* $(,)? ) => {
        $(
            // this took a while, please add const lowercase methods :crying:
            #[allow(unused)]
            pub const $Ident: Badge = Badge({
                const LIT: &str = stringify!($Ident);

                const ARR: [u8; LIT.len()] = const {
                    let mut arr: [u8; LIT.len()] = *LIT.as_bytes().as_array().unwrap();
                    arr.make_ascii_lowercase();

                    let mut i = 0;
                    while i < arr.len() {
                        if arr[i] == b'_' {
                            arr[i] = b' '
                        }

                        i += 1;
                    }

                    arr
                };

                unsafe {
                    str::from_utf8_unchecked(ARR.as_slice())
                }
            }, $color);
        )*
    };
}

badges! {
    RUST => "#f7a87e",
    SYN => "#cd516c",
    QUOTE => "#9761ca",
    WGPU => "#0089eb",
    WINIT => "#e0b944",
    ASTRO => "#e3399a",
    TAILWIND => "#14c1ca",
    C => "#3996e3",
    AVR8 => "#f35446",
    UBX => "#ff4b4b",
    BLENDER => "#ffa754",
    UNREAL_ENGINE => "#888bed",
    AXUM => "#f94fbe",
    PYTHON => "#51a5e6",
    COOLIFY => "#9658ff",
    DOCKER => "#2291e5",
    REACT => "#5e8aea",
    VITE => "#ffbe16",
    JWT => "#dd2f6f",
}

#[derive(Clone, Copy)]
enum Target {
    Here,
    Blank,
}

impl Target {
    fn value(self) -> Option<&'static str> {
        match self {
            Self::Here => None,
            Self::Blank => Some("_blank"),
        }
    }
}

pub fn layout(title: &str, path: &str, content: impl Render) -> String {
    let pages = [
        ("home", "/", Target::Here),
        // ("blog", "/blog", Target::Here),
        (
            "resume",
            "https://github.com/din0x/resume/raw/refs/heads/main/main.pdf",
            Target::Blank,
        ),
        ("github", "https://github.com/din0x", Target::Blank),
    ];

    html! {
        (DOCTYPE)
        html ."scheme-only-dark" {
            head {
                meta charset: "UTF-8";
                meta name: "viewport" content: "width=device-width, initial-scale=1.0";
                title { (title) }
                link rel: "icon" r#type: "image/svg+xml" href: "/assets/r.svg";
                link rel: "stylesheet" href: "/assets/css.css";
                script src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
            }
            body ."bg-mist-950 text-mist-300" {
                div ."px-6 w-full flex flex-col items-center bg-gray-900 border-b-1 border-gray-800" {
                    div ."py-4 w-full max-w-180" {
                        nav ."px-2 flex gap-8 text-mist-300 text-xl justify-end" {
                            for (name, href, target) in pages {
                                (nav_link(name, href, target, href == path))
                            }
                        }
                    }
                }
                div ."min-h-dvh" {
                    (content)
                }
                (footer())
            }
        }
    }
    .to_string()
}

fn nav_link(name: &str, href: &str, target: Target, is_active: bool) -> impl Render {
    html! {
        let text = (is_active)
            .then_some("text-blue-400 font-medium")
            .unwrap_or("hover:underline");

        a .(format!("decoration-2 decoration-blue-400 {text}"))
            href: (href)
            target: (target.value())
        {
            (name)
        }
    }
}

fn footer() -> impl Render {
    html! {
        div ."px-6 w-full flex flex-col items-center bg-gray-900 border-t-1 border-gray-800" {
            div ."py-4 w-full max-w-240 py-1 px-2 text-mist-300 text-xl" {
                p ."text-center text-blue-400 font-medium decoration-2 decoration-blue-400 cursor-pointer hover:underline"  onclick: "window.scrollTo({ top: 0, behavior: 'smooth' });" {
                    "Back to the top "
                    span ."font-mono" { "↑" }
                }
            }
        }
    }
}
