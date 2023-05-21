use crate::components::carousel::projects::Minor_Project;

pub const MINOR_PROJECTS: &[Minor_Project] = &[
    JUNGLE, 
    CHATTYAPP,
    CONLIGO,
    TWEETER,
    ];

const JUNGLE: Minor_Project = Minor_Project {
    name: "Jungle",
    github: "https://github.com/DraconianLore/jungle-rails",
    description: "An e-commerce application built with Ruby on rails."
};

const CHATTYAPP: Minor_Project = Minor_Project {
    name: "ChattyApp",
    github: "https://github.com/DraconianLore/chattyApp",
    description: "A compact anonymous chatroom built with nodeJS, React, WebSockets and sass."
};

const CONLIGO: Minor_Project = Minor_Project {
    name: "Conligo",
    github: "https://github.com/wonseobshin/conligo",
    description: "A smart ToDo list that categorizes items the user adds into a category by calling various APIs to find out what the item is."
};

const TWEETER: Minor_Project = Minor_Project {
    name: "Tweeter",
    github: "https://github.com/DraconianLore/tweeter",
    description: "A single-page AJAX-based Twitter clone that uses jQuery, HTML5 and CSS3."
};
