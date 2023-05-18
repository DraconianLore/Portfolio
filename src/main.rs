#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod components {
    pub mod header;
    pub mod carousel;
}


fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
// href="https://fonts.googleapis.com/css2?family=Bangers&display=swap" rel="stylesheet">
// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let main_css = include_str!("./styles.css");
    cx.render(rsx! {
        link {href: "https://fonts.googleapis.com/css2?family=Exo&display=swap", rel: "stylesheet"}
        style { main_css }
        components::header::Header{}
        div {
            id: "page_content",
            components::carousel::Carousel{}
        }
        
    })
}
