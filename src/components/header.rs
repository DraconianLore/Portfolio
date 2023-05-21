use dioxus::prelude::*;
pub mod style;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div { id: "header",
            style: style::HEADER,
            div { 
                style: style::HEADER_CONTENT,
                h1 {
                    "Steven Wing - Portfolio"
                }
                h1 {
                    a {
                        style: "text-decoration: none;",
                        href: "https://www.stevenwing.dev",
                        "Résumé"
                    }
                }
            }
        }
    })
}

