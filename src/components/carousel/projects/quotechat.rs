use crate::components::carousel::projects::Project;
use crate::components::carousel::projects::logos;
pub fn details() -> Project {
    Project{
        name: String::from("Quote-Chat"),
        description: String::from("A slack app using slash commands.\nOnce installed in the users workspace, the user canuse the following commands:\n'/quote -help' for instructions\n'/quote [some quote here]' to find and display 3 quotes to select from\nIf the user doesnt find a quote they loke they can press the shuffle button to find more."),
        tech_stack: String::from("Express, Node.js, Puthon, PostgreSQL"),
        image: String::from("quoteChat.gif"),
        github: String::from("https://github.com/alumni-lab/quote-chat"),
        live: String::from(""),
        logos: vec![logos::get_logo("express"), logos::get_logo("node.js"), logos::get_logo("python"), logos::get_logo("postgresql")],
    }
}

