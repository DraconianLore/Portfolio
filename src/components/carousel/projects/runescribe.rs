use crate::components::carousel::projects::{Project, logos};
pub fn details() -> Project {
    Project{
        name: String::from("Runescribe Familiar"),
        description: String::from("A tool for players using the custom Rune Scribe class I created for Dungeons & Dragons 5th Edition.\nThis tool helps players navigate the complexities of the new magic system created for this class, and allows players to add notes and tags to each spell, as well as filter by tag or type.\nThe admin side includes websockets connections so the Admin user can send messages to online players, add new structures and notify players in realtime, increase players/party level and get level-up choices from each player to decide what to unlock for each player as they level up."),
        tech_stack: String::from("Ruby, Rails, React, Websockets, PostgreSQL"),
        image: String::from("runescribe.webm"),
        github: String::from("https://github.com/DraconianLore/rune_scribe"),
        live: None,
        logos: vec![logos::RUBY, logos::RAILS, logos::REACT, logos::WEBSOCKETS, logos::POSTGRES],
    }
}
