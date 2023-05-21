use crate::components::carousel::projects::Project;
pub const DETAILS: Project = Project {
    name: "Quote-Chat",
    description: "A slack app using slash commands.\nOnce installed in the users workspace, the user canuse the following commands:\n'/quote -help' for instructions\n'/quote [some quote here]' to find and display 3 quotes to select from\nIf the user doesnt find a quote they loke they can press the shuffle button to find more.",
    tech_stack: "Express, Node.js, Puthon, PostgreSQL",
    image: "quoteChat.gif",
    github: "https://github.com/alumni-lab/quote-chat",
    live: "",
    logos: &[&["Express", "express.png", "https://expressjs.com"], &["Node.JS","nodejs-dark.png", "https://nodejs.org"],&["Python","python-logo.png","https://www.python.org"], &["PostgreSQL", "psql.svg", "https://www.postgresql.org"]],
};
