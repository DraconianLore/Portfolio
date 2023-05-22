use crate::components::carousel::projects::Project;
use crate::components::carousel::projects::logos;
pub fn details() -> Project {
    Project{
        name: String::from("DM Player Tracker"),
        description: String::from("A tool for Dungeon Masters to track their players and NPC's in Dungeons and Dragons.\n The user can see an overview of all their players and NPCs, and click into any character to see the full details.\n They can also level up characters, which auto-fills certain fields, manage player items and notes, and share a player version of characters with the players."),
        tech_stack: String::from("Ruby, Rails, React, PostgreSQL"),
        image: String::from("dmplayertracker.gif"),
        github: String::from("https://github.com/DraconianLore/dmPlayerTracker"),
        live: String::from("https://dmpt.stevenwing.dev/"),
        logos: vec![logos::get_logo("ruby"), logos::get_logo("rails"), logos::get_logo("react"), logos::get_logo("postgresql")],
    }
}
