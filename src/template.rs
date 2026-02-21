use gen_html::{Render, html};

pub fn link(r: impl Render, href: &str) -> impl Render {
    html! {
        // a ."group relative z-1" href: (href) target: "_blank" {
        //     span ."absolute left-0 bottom-0 -z-1 block w-full h-[2px] bg-red-400 duration-50
        //         group-hover:h-2" {}
        //     (r)
        // }
            a
        ."relative inline bg-gradient-to-t from-red-400 to-red-400
            bg-size-[100%_2px] bg-no-repeat bg-left-bottom
            duration-50
            hover:bg-size-[100%_100%]"
        href: (href)
        target: "_blank"
    {
        (r)
    }
    }
}
