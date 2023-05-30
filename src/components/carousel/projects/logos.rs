#[derive(Clone)]
pub struct Logo {
    pub title: &'static str,
    pub image: &'static str,
    pub link: &'static str,
}


pub const RUBY: Logo = Logo {
    title: "Ruby",
    image: "ruby.png",
    link: "https://www.ruby-lang.org",
};
pub const RUST: Logo = Logo {
    title: "Rust",
    image: "rust-logo-64x64.png",
    link: "https://www.rust-lang.org",
};
pub const RAILS: Logo = Logo {
    title: "Rails",
    image: "rails.png",
    link: "https://rubyonrails.org",
};
pub const REACT: Logo = Logo {
    title: "React",
    image: "react.png",
    link: "https://react.dev",
};
pub const REACT_NATIVE: Logo = Logo {
    title: "React Native",
    image: "react-native.png",
    link: "https://reactnative.dev",
};
pub const POSTGRES: Logo = Logo {
    title: "PostgreSQL",
    image: "psql.svg",
    link: "https://www.postgresql.org",
};
pub const WASM: Logo = Logo {
    title: "WebAssembly",
    image: "wasm.png",
    link: "https://webassembly.org",
};
pub const DIOXUS: Logo = Logo {
    title: "Dioxus",
    image: "dioxus.png",
    link: "https://dioxuslabs.com",
};
pub const WEBSOCKETS: Logo = Logo {
    title: "Websockets",
    image: "websockets.svg",
    link: "https://websockets.readthedocs.io",
};

pub const EXPRESS: Logo = Logo {
    title: "Express",
    image: "express.png",
    link: "https://expressjs.com",
};

pub const NODEJS: Logo = Logo {
    title: "Node.JS",
    image: "nodejs-dark.png",
    link: "https://nodejs.org",
};
pub const PYTHON: Logo = Logo {
    title: "Python",
    image: "python-logo.png",
    link: "https://www.python.org",
};
pub const ANGULAR: Logo = Logo {
    title: "Angular",
    image: "angular.png",
    link: "https://angular.io",
};

