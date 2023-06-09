use crate::components::carousel::projects::{Project, logos};

pub fn details() -> Project {
    Project{
        name: String::from("Koios"),
        description: String::from("Κοῖος is an immersive game where users enlist as agents in a secret organization where they are encouraged to take a break from their daily lives and complete missions by interacting with the world around them.\nInitially users are presented with basic training missions to build up trust within the organisation, taking photos of objects, taking selfies with other people or places, and sending them off to be verified by other agents(including new agents)\n\nAs the agents gain trust, they gain ranks in the organisation which leads to harder missions including:\nEncryption missions - where the agent is given a word or phrase and must encrypt it using a type of cypher provided.\nDecryption missions - where the agent is given an encrypted message and must discover the type of encryption used and decypher the message."),
        tech_stack: String::from("Ruby, Rails, React Native, PostgreSQL"),
        image: String::from("koios.png"),
        github: String::from("https://github.com/DraconianLore/Koios"),
        live: None,
        logos: vec![logos::RUBY, logos::RAILS, logos::REACT_NATIVE, logos::POSTGRES],
    }
}
