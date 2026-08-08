use gen_html::{Raw, Render, html};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::{fmt::Write, iter};

pub fn render(markdown: &str) -> Raw<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);

    let mut quote_level = 0u32;
    let mut s = String::new();

    for ev in parser {
        match ev {
            Event::Start(Tag::Strong) => {
                s.push_str(r#"<strong class="text-mist-300 font-medium">"#)
            }
            Event::Start(Tag::Paragraph) => {
                let border = if quote_level == 0 {
                    "border-l-4 border-red-400 border-dotted pl-2"
                } else {
                    ""
                };

                _ = write!(s, r#"<p class="mb-4 {border} font-serif">"#);
            }
            Event::Start(Tag::Heading {
                level,
                id,
                classes: _,
                attrs,
            }) => {
                let size = text_size(level);

                _ = write!(
                    s,
                    r#"<{level} class="mb-4 mt-10 border-l-4 border-red-400 pl-2 {size} text-mist-200 font-semibold font-serif""#
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
            }
            Event::Start(Tag::Link {
                link_type: _,
                dest_url,
                title: _,
                id: _,
            }) => {
                _ = write!(
                    s,
                    r#"<a class="2 text-mist-300 decoration-2 decoration-red-400 underline hover:text-mist-200 duration-200" href="{dest_url}">"#
                );
            }
            Event::Start(Tag::BlockQuote(_)) => {
                _ = write!(
                    s,
                    r#"<blockquote class="pl-2 border-mist-500 border-l-4 border-dotted text-mist-500 italic">"#
                );
                quote_level += 1;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                _ = write!(s, r#"</blockquote>"#);
                quote_level -= 1;
            }
            Event::Code(code) => {
                s.push_str(
                    &html! {
                        code ."text-blue-400" {
                            (code.to_string())
                        }
                    }
                    .render()
                    .0,
                );
            }
            Event::Start(Tag::CodeBlock(_)) => {
                _ = write!(
                    s,
                    r#"<pre class="mb-4"><code class="text-blue-400 border-l-4 border-blue-400 border-dotted pl-2">"#
                );
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
        HeadingLevel::H1 => "text-[1.30em]",
        HeadingLevel::H2 => "text-[1.25em]",
        HeadingLevel::H3 => "text-[1.20em]",
        HeadingLevel::H4 => "text-[1.15em]",
        HeadingLevel::H5 => "text-[1.10em]",
        HeadingLevel::H6 => "text-[1.05em]",
    }
}

fn render_default(ev: Event, s: &mut String) {
    pulldown_cmark::html::push_html(s, iter::once(ev));
}
