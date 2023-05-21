use crate::components::carousel::projects::Project;
pub const DETAILS: Project = Project {
    name: "Koios",
    description: "Κοῖος is an immersive game where users enlist as agents in a secret organization where they are encouraged to take a break from their daily lives and complete missions by interacting with the world around them.\nInitially users are presented with basic training missions to build up trust within the organisation, taking photos of objects, taking selfies with other people or places, and sending them off to be verified by other agents(including new agents)\n\nAs the agents gain trust, they gain ranks in the organisation which leads to harder missions including:\nEncryption missions - where the agent is given a word or phrase and must encrypt it using a type of cypher provided.\nDecryption missions - where the agent is given an encrypted message and must discover the type of encryption used and decypher the message.\n",
    tech_stack: "Ruby, Rails, React Native, PostgreSQL",
    image: "koios.png",
    github: "https://github.com/DraconianLore/Koios",
    live: "",
    logos: &[&["Ruby","ruby.png","https://www.ruby-lang.org"],&["Rails", "rails.png", "https://rubyonrails.org"], &["React Native", "react-native.png", "https://reactnative.dev/"], &["PostgreSQL", "psql.svg", "https://www.postgresql.org"]],
};
