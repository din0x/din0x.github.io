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
