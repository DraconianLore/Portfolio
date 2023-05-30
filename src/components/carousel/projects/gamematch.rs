use crate::components::carousel::projects::{Project, logos};

pub fn details() -> Project {
    Project{
        name: String::from("GameMatch"),
        description: String::from("A tool for multiple steam users to find multiplayer games they have in common.\n Users can import details from steam includin their friends list and games list.\nUsers select friends and quickly see what type of game - co-op or pvp, they share, or games that most of the selected friends have.\n If a game is on sale on steam the discount amount will also show."),
        tech_stack: String::from("Ruby, Rails, React, PostgreSQL"),
        image: String::from("GameMatch.webm"),
        github: String::from("https://github.com/DraconianLore/GameLibrary"),
        live: Some(String::from("https://gamematch.stevenwing.dev/")),
        logos: vec![logos::RUBY, logos::RAILS, logos::REACT, logos::POSTGRES],
    }
}
