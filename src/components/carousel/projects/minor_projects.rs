use crate::components::carousel::projects::MinorProject;

pub fn minro_projects() -> Vec<MinorProject> {
    vec![
        jungle(),
        chattyapp(),
        conligo(),
        tweeter(),
    ]
}

fn jungle() -> MinorProject {
    MinorProject {
        name: String::from("Jungle"),
        github: String::from("https://github.com/DraconianLore/jungle-rails"),
        description: String::from("An e-commerce application built with Ruby on Rails."),
    }
}

pub fn chattyapp() -> MinorProject {
    MinorProject {
        name: String::from("ChattyApp"),
        github: String::from("https://github.com/DraconianLore/chattyApp"),
        description: String::from("A compact anonymous chatroom built with Node.js, React, WebSockets, and Sass."),
    }
}

pub fn conligo() -> MinorProject {
    MinorProject {
        name: String::from("Conligo"),
        github: String::from("https://github.com/wonseobshin/conligo"),
        description: String::from("A smart ToDo list that categorizes items the user adds into a category by calling various APIs to find out what the item is."),
    }
}

pub fn tweeter() -> MinorProject {
    MinorProject {
        name: String::from("Tweeter"),
        github: String::from("https://github.com/DraconianLore/tweeter"),
        description: String::from("A single-page AJAX-based Twitter clone that uses jQuery, HTML5, and CSS3."),
    }
}
