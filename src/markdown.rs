use gen_html::{Raw, Render, html};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::{fmt::Write, iter};

pub fn render(markdown: &str) -> Raw<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);

    let mut s = String::new();

    for ev in parser {
        match ev {
            Event::Start(Tag::Strong) => {
                s.push_str(r#"<strong class="text-mist-300 font-medium">"#)
            }
            Event::Start(Tag::Paragraph) => s.push_str(r#"<p class="mb-8">"#),
            Event::Start(Tag::Heading {
                level,
                id,
                classes: _,
                attrs,
            }) => {
                let size = text_size(level);

                _ = write!(
                    s,
                    r#"<{level} class="mb-4 {size} text-mist-200 font-medium""#
                );

                if let Some(id) = id {
                    _ = write!(s, r#" id="{id}""#);
                }

                for (name, value) in attrs {
                    _ = write!(s, " {name}");

                    if let Some(value) = value {
                        _ = write!(s, "=\"{value}\"");
                    }
                }
                _ = write!(s, ">");

                s.push_str(
                    &html! {
                        span ."text-red-400" {
                            for _ in 0..level as u8 {
                                "#"
                            }
                        }
                        " "
                    }
                    .render()
                    .0,
                );
            }
            Event::Start(Tag::Link {
                link_type: _,
                dest_url,
                title: _,
                id: _,
            }) => {
                _ = write!(
                    s,
                    r#"<a class="text-red-400 decoration-2 decoration-red-400 hover:underline" href="{dest_url}">"#
                );
            }
            Event::Start(Tag::BlockQuote(_)) => {
                _ = write!(
                    s,
                    r#"<blockquote class="pl-2 border-mist-500 border-l-3 text-mist-500">"#
                );
            }
            Event::Code(code) => {
                s.push_str(
                    &html! {
                        code ."text-blue-400" {
                            "`"
                            (code.to_string())
                            "`"
                        }
                    }
                    .render()
                    .0,
                );
            }
            Event::Start(Tag::CodeBlock(_)) => {
                _ = write!(s, "<pre class=\"mb-8\"><code class=\"text-blue-400\">");
            }
            Event::End(TagEnd::CodeBlock) => {
                _ = write!(s, "</code></pre>");
            }
            ev => render_default(ev, &mut s),
        }
    }

    Raw(s)
}

fn text_size(h: HeadingLevel) -> &'static str {
    match h {
        HeadingLevel::H1 => "text-[2em]",
        HeadingLevel::H2 => "text-[1.75em]",
        HeadingLevel::H3 => "text-[1.5em]",
        HeadingLevel::H4 => "text-[1.25em]",
        HeadingLevel::H5 => "text-[1.125em]",
        HeadingLevel::H6 => "text-[1em]",
    }
}

fn render_default(ev: Event, s: &mut String) {
    pulldown_cmark::html::push_html(s, iter::once(ev));
}
