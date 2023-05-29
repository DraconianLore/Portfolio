use crate::components::carousel::projects::logos::Logo;

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
pub fn GET_PROJECT(i: i32) -> Project {
    let current_project = match i {
        0 => portfolio::details(),
        1 => dmpt::details(),
        2 => gamematch::details(),
        3 => runescribe::details(),
        4 => quotechat::details(),
        5 => donateIT::details(),
        6 => koios::details(),
        _ => portfolio::details(),
    };
    return current_project;
}
