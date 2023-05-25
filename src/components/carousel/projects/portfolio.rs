use crate::components::carousel::projects::Project;
use crate::components::carousel::projects::logos;
pub fn details() -> Project {
    Project{
        name: String::from("My Portfolio"),
        description: String::from("A single-page app version of my portfolio utilizing new technologies to show off my versatility, ability to learn new languages and frameworks, and showcase my other projects."),
        tech_stack: String::from("Rust, Dioxus, WebAssembly"),
        image: String::from("portfolio.png"),
        github: String::from("https://github.com/DraconianLore/Portfolio"),
        live: String::from("https://www.stevenwing.dev/"),
        logos: vec![logos::get_logo("rust"), logos::get_logo("dioxus"), logos::get_logo("webassembly")],
    }
}
