use gen_html::{Render, html};

pub fn link(r: impl Render, href: &str) -> impl Render {
    html! {
        a ."group relative" href: (href) target: "_blank" {
            span ."absolute left-0 bottom-0 -z-1 block w-full h-[2px] bg-red-400 duration-50 z-1
                group-hover:h-1" {}
            (r)
        }
    }
}
