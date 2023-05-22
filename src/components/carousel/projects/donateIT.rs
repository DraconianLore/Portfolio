use crate::components::carousel::projects::Project;
use crate::components::carousel::projects::logos;

pub fn details() -> Project {
    Project{
        name: String::from("Donate IT"),
        description: String::from("Created during the 2019 Vanhacks < For Social Good > hackathon.\nThis application allows not for profit organizations to post their IT problems such as 'We need a website' for developers/designers to match with to donate their time and experience to helpin out their community."),
        tech_stack: String::from("Ruby, Rails, Angular, PostgreSQL"),
        image: String::from("donateIT.gif"),
        github: String::from("https://github.com/ChesterCorin/vanhacks-2019-frontend"),
        live: String::from(""),
        logos: vec![logos::get_logo("ruby"), logos::get_logo("rails"), logos::get_logo("angular"), logos::get_logo("postgresql")]
    }
}
