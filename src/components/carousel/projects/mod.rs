use crate::components::carousel::projects::logos::Logo;
#[derive(Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub tech_stack: String,
    pub image: String,
    pub github: String,
    pub live: Option<String>,
    pub logos: Vec<Logo>,
}

pub struct MinorProject {
    pub name: String,
    pub github: String,
    pub description: String,
}

pub mod logos;

// Import project files here
pub mod minor_projects;
pub mod portfolio;
pub mod dmpt;
pub mod gamematch;
pub mod quotechat;
pub mod donateIT;
pub mod koios;
pub mod runescribe;

// Add new projects here in desired slide order

pub fn load_projects() -> Vec<Project> {
    vec![
        portfolio::details(), 
        dmpt::details(), 
        gamematch::details(),
        runescribe::details(),
        quotechat::details(),
        donateIT::details(),
        koios::details()
        ]
}
