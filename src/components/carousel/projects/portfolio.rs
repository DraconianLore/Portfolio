use crate::components::carousel::projects::{Project, logos};

pub fn details() -> Project {
    Project{
        name: String::from("My Portfolio"),
        description: String::from("A single-page app version of my portfolio utilizing new technologies to show off my versatility, ability to learn new languages and frameworks, and showcase my other projects."),
        tech_stack: String::from("Rust, Dioxus, WebAssembly"),
        image: String::from("portfolio.webm"),
        github: String::from("https://github.com/DraconianLore/Portfolio"),
        live: None,
        logos: vec![logos::RUST, logos::DIOXUS, logos::WASM],
    }
}
