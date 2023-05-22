pub struct Logo {
    pub title: String,
    pub image: String,
    pub link: String,
}

pub fn get_logo(logo: &str) -> Logo {
    let ruby = Logo {
        title: String::from("Ruby"),
        image: String::from("ruby.png"),
        link: String::from("https://www.ruby-lang.org"),
    };
    let rust = Logo {
        title: String::from("Rust"),
        image: String::from("rust-logo-64x64.png"),
        link: String::from("https://www.rust-lang.org"),
    };
    let rails = Logo {
        title: String::from("Rails"),
        image: String::from("rails.png"),
        link: String::from("https://rubyonrails.org"),
    };
    let react = Logo {
        title: String::from("React"),
        image: String::from("react.png"),
        link: String::from("https://react.dev"),
    };
    let react_native = Logo {
        title: String::from("React Native"),
        image: String::from("react-native.png"),
        link: String::from("https://reactnative.dev"),
    };
    let postgres = Logo {
        title: String::from("PostgreSQL"),
        image: String::from("psql.svg"),
        link: String::from("https://www.postgresql.org"),
    };
    let wasm = Logo {
        title: String::from("WebAssembly"),
        image: String::from("wasm.png"),
        link: String::from("https://webassembly.org"),
    };
    let dioxus = Logo {
        title: String::from("Dioxus"),
        image: String::from("dioxus.png"),
        link: String::from("https://dioxuslabs.com"),
    };
    let websockets = Logo {
        title: String::from("Websockets"),
        image: String::from("websockets.svg"),
        link: String::from("https://websockets.readthedocs.io"),
    };
    
    let express = Logo {
        title: String::from("Express"),
        image: String::from("express.png"),
        link: String::from("https://expressjs.com"),
    };
    
    let nodejs = Logo {
        title: String::from("Node.JS"),
        image: String::from("nodejs-dark.png"),
        link: String::from("https://nodejs.org"),
    };
    let python = Logo {
        title: String::from("Python"),
        image: String::from("python-logo.png"),
        link: String::from("https://www.python.org"),
    };
    let angular = Logo {
        title: String::from("Angular"),
        image: String::from("angular.png"),
        link: String::from("https://angular.io"),
    };

    match logo {
        "ruby" => ruby,
        "rust" => rust,
        "rails" => rails,
        "react" => react,
        "react native" => react_native,
        "postgresql" => postgres,
        "webassembly" => wasm,
        "dioxus" => dioxus,
        "websockets" => websockets,
        "express" => express,
        "node.js" => nodejs,
        "python" => python,
        "angular" => angular,
        _ => react,
    }
}
