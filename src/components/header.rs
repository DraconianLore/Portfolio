use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    let styling = include_str!("./header.css");
    cx.render(rsx! {
        style { styling }
        div { id: "header",
            div { 
                class: "headerContent",
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

