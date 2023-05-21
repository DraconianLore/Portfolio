use crate::components::carousel::projects::Project;
pub const DETAILS: Project = Project {
    name: "DM Player Tracker",
    description: "A tool for Dungeon Masters to track their players and NPC's in Dungeons and Dragons.\n The user can see an overview of all their players and NPCs, and click into any character to see the full details.\n They can also level up characters, which auto-fills certain fields, manage player items and notes, and share a player version of characters with the players.",
    tech_stack: "Ruby, Rails, React, PostgreSQL",
    image: "dmplayertracker.gif",
    github: "https://github.com/DraconianLore/dmPlayerTracker",
    live: "https://dmpt.stevenwing.dev/",
    logos: &[&["Ruby","ruby.png","https://www.ruby-lang.org"],&["Ruby on Rails", "rails.png", "https://rubyonrails.org"], &["React", "react.png", "https://react.dev"], &["PostgreSQL", "psql.svg", "https://www.postgresql.org"]],
};
