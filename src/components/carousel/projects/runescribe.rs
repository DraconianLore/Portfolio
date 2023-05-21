use crate::components::carousel::projects::Project;
pub const DETAILS: Project = Project {
    name: "Runescribe Familiar",
    description: "A tool for players using the custom Rune Scribe class I created for Dungeons & Dragons 5th Edition.\nThis tool helps players navigate the complexities of the new magic system created for this class, and allows players to add notes and tags to each spell, as well as filter by tag or type.\nThe admin side includes websockets connections so the Admin user can send messages to online players, add new structures and notify players in realtime, increase players/party level and get level-up choices from each player to decide what to unlock for each player as they level up.",
    tech_stack: "Ruby, Rails, React, Websockets, PostgreSQL",
    image: "runescribe.webm",
    github: "https://github.com/DraconianLore/rune_scribe",
    live: "",
    logos: &[&["Ruby","ruby.png","https://www.ruby-lang.org"],&["Rails", "rails.png", "https://rubyonrails.org"], &["React", "react.png", "https://react.dev"], &["Websockets", "websockets.svg", "https://websockets.readthedocs.io"], &["PostgreSQL", "psql.svg", "https://www.postgresql.org"]],
};
