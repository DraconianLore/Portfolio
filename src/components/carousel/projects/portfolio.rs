use crate::components::carousel::projects::Project;
// use crate::compon ents::carousel::projects::Logos;
pub static DETAILS: Project = Project {
    name: "My Portfolio",
    description: "A single-page app version of my portfolio utilizing new technologies to show off my versatility, ability to learn new languages and frameworks, and showcase my other projects.",
    tech_stack: "Rust, Dioxus, WebAssembly",
    image: "portfolio.png",
    github: "https://github.com/DraconianLore/Portfolio",
    live: "https://www.stevenwing.dev/",
    logos: &[&["Rust", "rust-logo-64x64.png", "https://www.rust-lang.org"], &["Dioxus", "dioxus.png", "https://dioxuslabs.com"], &["WebAssembly", "wasm.png", "https://webassembly.org"]],
    // logos: logos,
};

// pub static logos = Vec::from([&["Rust", "rust-logo-64x64.png", "https://www.rust-lang.org"], &["Dioxus", "dioxus.png", "https://dioxuslabs.com"], &["WebAssembly", "wasm.png", "https://webassembly.org"]]);