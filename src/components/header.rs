use dioxus::prelude::*;
pub mod style;

pub fn Header(cx: Scope) -> Element {
    // let styling = include_str!("./header.css");


    cx.render(rsx! {
        // style { styling }
        div { id: "header",
            style: style::HEADER,
            div { 
                style: style::HEADER_CONTENT,
                h1 {
                    "Steven Wing - Portfolio"
                }
                h1 {
                    "test"
                }
            }
        }
    })
}

