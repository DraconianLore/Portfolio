// Structure of Project
// struct Logos {
//     name: &str,
//     image: &str,
//     link: &str,
// }
pub struct Project<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub tech_stack: &'a str,
    pub image: &'a str,
    pub github: &'a str,
    pub live: &'a str,
    pub logos: &'static [&'static [&'static str; 3]],
    // pub logos: Vec<Logo>,
}

pub struct Minor_Project<'a> {
    pub name: &'a str,
    pub github: &'a str,
    pub description: &'a str,
}

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
pub fn GET_PROJECT(i: i32) -> &'static Project<'static> {
    let current_project = match i{
        0=>&portfolio::DETAILS,
        1=>&dmpt::DETAILS,
        2=>&gamematch::DETAILS,
        3=>&runescribe::DETAILS,
        4=>&quotechat::DETAILS,
        5=>&donateIT::DETAILS,
        6=>&koios::DETAILS,
        _=>&portfolio::DETAILS,
    };
    return current_project
}
