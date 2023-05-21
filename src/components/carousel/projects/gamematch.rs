use crate::components::carousel::projects::Project;
pub const DETAILS: Project = Project {
    name: "GameMatch",
    description: "A tool for multiple steam users to find multiplayer games they have in common.\n Users can import details from steam includin their friends list and games list.\nUsers select friends and quickly see what type of game - co-op or pvp, they share, or games that most of the selected friends have.\n If a game is on sale on steam the discount amount will also show.",
    tech_stack: "Ruby, Rails, React, PostgreSQL",
    image: "GameMatch.webm",
    github: "https://github.com/DraconianLore/GameLibrary",
    live: "https://gamematch.stevenwing.dev/",
    logos: &[&["Ruby","ruby.png","https://www.ruby-lang.org"],&["Rails", "rails.png", "https://rubyonrails.org"], &["React", "react.png", "https://react.dev"], &["PostgreSQL", "psql.svg", "https://www.postgresql.org"]],
};
