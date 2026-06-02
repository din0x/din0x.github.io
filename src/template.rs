use std::fmt;

use gen_html::{Render, html};

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
        div ."p-2 mb-2 rounded-lg border-2 text-mist-400 border-mist-800 bg-mist-900 break-inside-avoid" {
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
    TAILWIND => "#74d4ff",
    C => "#3996e3",
    AVR8 => "#f35446",
    UBX => "#ff4b4b",
    BLENDER => "#ffa754",
    UNREAL_ENGINE => "#888bed"
}
