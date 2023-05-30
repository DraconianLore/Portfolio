use crate::components::carousel::projects::{Project, logos};

pub fn details() -> Project {
    Project{
        name: String::from("DM Player Tracker"),
        description: String::from("A tool for Dungeon Masters to track their players and NPC's in Dungeons and Dragons.\n The user can see an overview of all their players and NPCs, and click into any character to see the full details.\n They can also level up characters, which auto-fills certain fields, manage player items and notes, and share a player version of characters with the players."),
        tech_stack: String::from("Ruby, Rails, React, PostgreSQL"),
        image: String::from("dmplayertracker.gif"),
        github: String::from("https://github.com/DraconianLore/dmPlayerTracker"),
        live: Some(String::from("https://dmpt.stevenwing.dev/")),
        logos: vec![logos::RUBY, logos::RAILS, logos::REACT, logos::POSTGRES],
    }
}
